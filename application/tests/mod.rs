mod chat;
mod game;
mod monitor;
mod request;
mod response;
mod rss;
mod upload;
mod utility;

use hyperlane_application::{
    domain::{chat::ChatDomain, gomoku::GomokuDomain},
    mapper::monitor::PerformanceRingBuffer,
    model::{
        application::{
            chat::{ChatMessage, ChatSession, MessageType, OnlineUser},
            gomoku::{
                GameStatus, GomokuMessageType, GomokuPlaceResult, GomokuPlayer, GomokuRoom,
                StoneColor,
            },
            monitor::PerformanceDataPoint,
            rss::UploadedFile,
            upload::{FileChunkData, RangeRequest},
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
            common::{ApiResponse, ApiResponseStatus},
            gomoku::GomokuWsResponse,
            websocket::MessageResponse,
        },
    },
    service::{
        auth::{AuthService, PasswordUtil},
        chat::ChatService,
        euv_playground::{BuildJobId, EuvPlaygroundService},
        github_pages::{
            extract_resource_paths, extract_resource_paths_by_extension, is_safe_path,
            parse_range_header, read_file_range,
        },
        rss::RssService,
        upload::UploadService,
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
    env::{current_dir, current_exe, var, var_os},
    ffi::{OsStr, OsString},
    fs::{File, FileTimes, create_dir_all, metadata, read, write},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        BodyPart, Context, HttpVersion, Method, MultipartFormData, Request, RequestHeaders,
        hash_map_xx_hash3_64,
        serde_json::{Error, Value, from_slice, from_str, from_value, to_string, to_value},
    },
    hyperlane_config::application::upload::upload_dir,
    rsa::{
        RsaPrivateKey, RsaPublicKey,
        pkcs1::{EncodeRsaPublicKey, LineEnding},
        traits::PublicKeyParts,
    },
    tempfile::{TempDir, tempdir},
    urlencoding::decode,
};

const ERROR_BUSINESS_LOGIC: &str = "Business logic error";
const ERROR_CONFLICT: &str = "Conflict";
const ERROR_DATABASE: &str = "Database error";
const ERROR_EXTERNAL_SERVICE: &str = "External service error";
const ERROR_FORBIDDEN: &str = "Forbidden";
const ERROR_GAME_NOT_IN_PROGRESS: &str = "Game is not in progress";
const ERROR_INTERNAL_SERVER: &str = "Internal server error";
const ERROR_INVALID_END_RANGE: &str = "Invalid end range";
const ERROR_INVALID_ID_FORMAT: &str = "Invalid ID format";
const ERROR_INVALID_POSITION: &str = "Invalid position";
const ERROR_INVALID_RANGE_BOTH_EMPTY: &str = "Invalid range: both start and end are empty";
const ERROR_INVALID_RANGE_HEADER_FORMAT: &str = "Invalid range header format";
const ERROR_INVALID_RANGE_SPECIFICATION: &str = "Invalid range specification";
const ERROR_INVALID_RANGE_START_GREATER_THAN_END: &str = "Invalid range: start > end";
const ERROR_INVALID_REQUEST: &str = "Invalid request";
const ERROR_INVALID_START_RANGE: &str = "Invalid start range";
const ERROR_NOT_YOUR_TURN: &str = "Not your turn";
const ERROR_PLAYER_NOT_FOUND: &str = "Player not found";
const ERROR_POSITION_OCCUPIED: &str = "Position occupied";
const ERROR_RANGE_START_EXCEEDS_FILE_SIZE: &str = "Range start exceeds file size";
const ERROR_RATE_LIMIT_EXCEEDED: &str = "Rate limit exceeded";
const ERROR_REQUEST_TIMEOUT: &str = "Request timeout";
const ERROR_RESOURCE_NOT_FOUND: &str = "Resource not found";
const ERROR_ROOM_IS_FULL: &str = "Room is full";
const ERROR_UNAUTHORIZED: &str = "Unauthorized";
const ERROR_WAITING_FOR_SECOND_PLAYER: &str = "Waiting for second player";
const EUV_PLAYGROUND_CARGO_BIN_DIR: &str = "bin";
const EUV_PLAYGROUND_CARGO_HOME_DIR: &str = ".cargo";
const EUV_PLAYGROUND_MAX_NAME_LEN: usize = 64;
const EUV_PLAYGROUND_META_FILE: &str = "metadata.json";
const EUV_PLAYGROUND_SEQ_FILE: &str = "_seq";
#[cfg(windows)]
const EUV_PLAYGROUND_WASM_PACK_BINARY_NAME: &str = "wasm-pack.exe";
#[cfg(not(windows))]
const EUV_PLAYGROUND_WASM_PACK_BINARY_NAME: &str = "wasm-pack";
const GPT: &str = "gpt";
const HTTP_REASON_SUCCESS: &str = "Success";
const MAX_HISTORY_SECONDS: usize = 60;
const RESOURCE_EXTENSION_HTML: &str = "html";
const UNTITLED_PROJECT_NAME: &str = "Untitled";
