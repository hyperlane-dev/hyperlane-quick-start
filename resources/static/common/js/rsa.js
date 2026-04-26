var RsaCrypto = (function () {
  var jwkKey = null;

  async function fetchPublicKey() {
    try {
      var response = await fetch('/api/auth/rsa/public-key');
      var result = await response.json();
      if (result.code !== 200) {
        throw new Error(result.data || 'Failed to fetch public key');
      }
      var data = result.data;
      if (data.n && data.e) {
        jwkKey = {
          kty: 'RSA',
          n: data.n,
          e: data.e,
          alg: 'RSA-OAEP-256',
          ext: true,
        };
        return jwkKey;
      }
      if (data.public_key) {
        jwkKey = parsePemToJwk(data.public_key);
        return jwkKey;
      }
      throw new Error('Invalid public key format');
    } catch (error) {
      console.error('Failed to fetch RSA public key:', error);
      throw error;
    }
  }

  function parsePemToJwk(pem) {
    var b64 = pem
      .replace(/-----BEGIN RSA PUBLIC KEY-----/, '')
      .replace(/-----END RSA PUBLIC KEY-----/, '')
      .replace(/-----BEGIN PUBLIC KEY-----/, '')
      .replace(/-----END PUBLIC KEY-----/, '')
      .replace(/\s/g, '');
    var raw = base64UrlToUint8Array(b64);
    var der = asn1Parse(raw);
    if (der.tag === 0x30) {
      var seq = der.value;
      if (seq.length === 2 && seq[0].tag === 0x02 && seq[1].tag === 0x02) {
        return {
          kty: 'RSA',
          n: uint8ArrayToBase64Url(seq[0].value),
          e: uint8ArrayToBase64Url(seq[1].value),
          alg: 'RSA-OAEP-256',
          ext: true,
        };
      }
      if (seq.length >= 2 && seq[0].tag === 0x30 && seq[1].tag === 0x03) {
        var bitString = unpadBitString(seq[1].value);
        var inner = asn1Parse(bitString);
        if (inner.tag === 0x30 && inner.value.length >= 2) {
          var innerSeq = inner.value;
          return {
            kty: 'RSA',
            n: uint8ArrayToBase64Url(innerSeq[0].value),
            e: uint8ArrayToBase64Url(innerSeq[1].value),
            alg: 'RSA-OAEP-256',
            ext: true,
          };
        }
      }
    }
    throw new Error('Unable to parse RSA public key from PEM');
  }

  function asn1Parse(data) {
    var pos = 0;
    function readTag() {
      if (pos >= data.length) throw new Error('Unexpected end of ASN.1 data');
      var tag = data[pos++];
      var lenByte = data[pos++];
      var length = 0;
      if (lenByte & 0x80) {
        var numBytes = lenByte & 0x7f;
        for (var i = 0; i < numBytes; i++) {
          length = (length << 8) | data[pos++];
        }
      } else {
        length = lenByte;
      }
      var value = data.slice(pos, pos + length);
      pos += length;
      var children = [];
      if ((tag & 0x20) === 0x20) {
        var end = pos;
        pos -= length;
        while (pos < end) {
          children.push(readTag());
        }
        value = children;
      }
      return { tag: tag, value: value };
    }
    return readTag();
  }

  function unpadBitString(data) {
    if (data.length < 1) return new Uint8Array(0);
    var unusedBits = data[0];
    return data.slice(1, data.length - unusedBits);
  }

  function base64UrlToUint8Array(b64) {
    var padded = b64.replace(/-/g, '+').replace(/_/g, '/');
    while (padded.length % 4 !== 0) {
      padded += '=';
    }
    var bin = atob(padded);
    var bytes = new Uint8Array(bin.length);
    for (var i = 0; i < bin.length; i++) {
      bytes[i] = bin.charCodeAt(i);
    }
    return bytes;
  }

  function uint8ArrayToBase64Url(bytes) {
    var bin = '';
    for (var i = 0; i < bytes.length; i++) {
      bin += String.fromCharCode(bytes[i]);
    }
    var b64 = btoa(bin);
    return b64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
  }

  function pkcs1v15Pad(data, keySizeBytes) {
    var targetLength = keySizeBytes;
    if (data.length > targetLength - 11) {
      throw new Error('Message too long for RSA PKCS#1 v1.5 encryption');
    }
    var paddingLength = targetLength - 3 - data.length;
    var buffer = new Uint8Array(targetLength);
    buffer[0] = 0x00;
    buffer[1] = 0x02;
    for (var i = 2; i < 2 + paddingLength; i++) {
      buffer[i] = 0xff;
    }
    buffer[2 + paddingLength] = 0x00;
    var dataBytes = new TextEncoder().encode(data);
    buffer.set(dataBytes, 3 + paddingLength);
    return buffer;
  }

  function base64ToBigInt(b64) {
    var padded = b64.replace(/-/g, '+').replace(/_/g, '/');
    while (padded.length % 4 !== 0) {
      padded += '=';
    }
    var bin = atob(padded);
    var hex = '';
    for (var i = 0; i < bin.length; i++) {
      var h = bin.charCodeAt(i).toString(16);
      hex += h.length === 1 ? '0' + h : h;
    }
    return BigInt('0x' + hex);
  }

  function bigIntToUint8Array(bigint, byteLength) {
    var bytes = new Uint8Array(byteLength);
    var temp = bigint;
    for (var i = byteLength - 1; i >= 0; i--) {
      bytes[i] = Number(temp & 0xffn);
      temp >>= 8n;
    }
    return bytes;
  }

  function uint8ArrayToBigInt(bytes) {
    var result = 0n;
    for (var i = 0; i < bytes.length; i++) {
      result = (result << 8n) | BigInt(bytes[i]);
    }
    return result;
  }

  function modPow(base, exp, mod) {
    var result = 1n;
    base = base % mod;
    while (exp > 0n) {
      if (exp % 2n === 1n) {
        result = (result * base) % mod;
      }
      exp >>= 1n;
      base = (base * base) % mod;
    }
    return result;
  }

  async function encryptWithPkcs1v15(plaintext, jwk) {
    var nBigInt = base64ToBigInt(jwk.n);
    var eBigInt = base64ToBigInt(jwk.e);

    var paddedData = pkcs1v15Pad(
      plaintext,
      Math.ceil(nBigInt.toString(2).length / 8),
    );
    var m = uint8ArrayToBigInt(paddedData);
    if (m >= nBigInt) {
      throw new Error('Message too long for RSA key size');
    }
    var c = modPow(m, eBigInt, nBigInt);
    var keyByteLen = Math.ceil(nBigInt.toString(2).length / 8);
    var encryptedBytes = bigIntToUint8Array(c, keyByteLen);

    var binary = '';
    for (var i = 0; i < encryptedBytes.length; i++) {
      binary += String.fromCharCode(encryptedBytes[i]);
    }
    return btoa(binary);
  }

  async function encryptField(plaintext) {
    if (!jwkKey) {
      await fetchPublicKey();
    }
    if (!jwkKey) {
      throw new Error('Public key not available');
    }
    return encryptWithPkcs1v15(plaintext, jwkKey);
  }

  async function refreshPublicKey() {
    jwkKey = null;
    return fetchPublicKey();
  }

  function getPublicKey() {
    return jwkKey;
  }

  return {
    encryptField: encryptField,
    fetchPublicKey: fetchPublicKey,
    refreshPublicKey: refreshPublicKey,
    getPublicKey: getPublicKey,
  };
})();
