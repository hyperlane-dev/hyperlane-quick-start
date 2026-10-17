mod chat;
mod game;
mod monitor;
mod request;
mod response;
mod rss;
mod utility;

use hyperlane_application::{
    domain::{
        chat::ChatDomain,
        gomoku::{
            ERROR_GAME_NOT_IN_PROGRESS, ERROR_INVALID_POSITION, ERROR_NOT_YOUR_TURN,
            ERROR_PLAYER_NOT_FOUND, ERROR_POSITION_OCCUPIED, ERROR_ROOM_IS_FULL,
            ERROR_WAITING_FOR_SECOND_PLAYER, GomokuDomain,
        },
    },
    mapper::monitor::PerformanceRingBuffer,
    model::{
        application::{
            chat::{ChatMessage, ChatSession, GPT, MessageType, OnlineUser},
            gomoku::{
                GameStatus, GomokuMessageType, GomokuPlaceResult, GomokuPlayer, GomokuRoom,
                StoneColor,
            },
            monitor::{MAX_HISTORY_SECONDS, PerformanceDataPoint},
            rss::UploadedFile,
            upload::RangeRequest,
        },
        request::{
            chat::{ChatHistoryParams, WebSocketReqData},
            euv_playground::{
                EuvPlaygroundProjectCreateRequest, EuvPlaygroundProjectSaveRequest,
                EuvPlaygroundRunRequest,
            },
            gomoku::GomokuWsRequest,
            rss::{RssFeedRequest, Timezone},
            websocket::WebSocketMessage,
        },
        response::{
            chat::WebSocketRespData,
            common::{
                ApiResponse, ApiResponseStatus, ERROR_BUSINESS_LOGIC, ERROR_CONFLICT,
                ERROR_DATABASE, ERROR_EXTERNAL_SERVICE, ERROR_FORBIDDEN, ERROR_INTERNAL_SERVER,
                ERROR_INVALID_REQUEST, ERROR_RATE_LIMIT_EXCEEDED, ERROR_REQUEST_TIMEOUT,
                ERROR_RESOURCE_NOT_FOUND, ERROR_UNAUTHORIZED, HTTP_REASON_SUCCESS,
            },
            gomoku::GomokuWsResponse,
            websocket::MessageResponse,
        },
    },
    service::{
        self,
        auth::{AuthService, PasswordUtil},
        chat::ChatService,
        euv_playground::{
            BuildJobId, ERROR_INVALID_ID_FORMAT, EUV_PLAYGROUND_CARGO_BIN_DIR,
            EUV_PLAYGROUND_CARGO_HOME_DIR, EUV_PLAYGROUND_MAX_NAME_LEN, EUV_PLAYGROUND_META_FILE,
            EUV_PLAYGROUND_SEQ_FILE, EUV_PLAYGROUND_WASM_PACK_BINARY_NAME, EuvPlaygroundService,
            UNTITLED_PROJECT_NAME,
        },
        github_pages::{
            ERROR_INVALID_RANGE_SPECIFICATION, ERROR_INVALID_RANGE_START_GREATER_THAN_END,
            ERROR_RANGE_START_EXCEEDS_FILE_SIZE, RESOURCE_EXTENSION_HTML, extract_resource_paths,
            extract_resource_paths_by_extension, is_safe_path, parse_range_header, read_file_range,
        },
        rss::RssService,
        upload::{
            ERROR_INVALID_END_RANGE, ERROR_INVALID_RANGE_BOTH_EMPTY,
            ERROR_INVALID_RANGE_HEADER_FORMAT, ERROR_INVALID_START_RANGE, UploadService,
        },
    },
    utils::{
        auth::build_auth_redirect_url,
        content_type::format_content_type,
        crypto::RsaUtil,
        gzip::is_gzip_compressible,
        json::{get_request_json, get_response_json},
    },
};

use std::{
    collections::{HashMap, VecDeque},
    env::{current_dir, current_exe, var},
    ffi::{OsStr, OsString},
    fs::{File, FileTimes, create_dir_all, metadata, write},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        Context, Method,
        serde_json::{Error, Value, from_str, from_value, to_string, to_value},
    },
    hyperlane_config::application::upload::UPLOAD_DIR,
    rsa::{
        RsaPrivateKey, RsaPublicKey,
        pkcs1::{EncodeRsaPublicKey, LineEnding},
        traits::PublicKeyParts,
    },
    tempfile::{TempDir, tempdir},
    urlencoding::decode,
};
