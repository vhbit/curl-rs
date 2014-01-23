#[desc = "A rust package for libcurl."];
#[license = "MIT"];

use std::libc::{c_char, c_long, c_int};
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;

#[link(name = "curl")]
extern {
    fn curl_version() -> *c_char;
    fn curl_global_init(flags: c_long) -> c_int;
    fn curl_global_cleanup();
}

pub static GLOBAL_SSL : c_long = (1<<0);
pub static GLOBAL_WIN32 : c_long = (1<<1);
pub static GLOBAL_ALL : c_long = (GLOBAL_SSL|GLOBAL_WIN32);
pub static GLOBAL_NOTHING : c_long = 0;
pub static GLOBAL_DEFAULT : c_long = GLOBAL_ALL;
pub static GLOBAL_ACK_EINTR : c_long = (1<<2);

pub fn global_init(flags: c_long) -> int {
    unsafe { curl_global_init(flags) as int }
}

pub fn global_cleanup() {
    unsafe { curl_global_cleanup() }
}

pub fn version() -> ~str {
    unsafe {
        // for curl version, we don't own it
        let cver = CString::new(curl_version(), false);
        str::from_utf8_owned(cver.container_into_owned_bytes()).unwrap()
    }
}

pub mod easy;

pub mod opt {
    use std::libc::types::os::arch::c95::c_int;

    static LONG: c_int = 0;
    static OBJECTPOINT: c_int = 10_000;
    static FUNCTIONPOINT: c_int = 20_000;
    static OFF_T: c_int = 30_000;

    pub static FILE : c_int = OBJECTPOINT + 1;
    pub static URL : c_int = OBJECTPOINT + 2;
    pub static PORT : c_int = LONG + 3;
    pub static PROXY : c_int = OBJECTPOINT + 4;
    pub static USERPWD : c_int = OBJECTPOINT + 5;
    pub static PROXYUSERPWD : c_int = OBJECTPOINT + 6;
    pub static RANGE : c_int = OBJECTPOINT + 7;
    pub static INFILE : c_int = OBJECTPOINT + 9;
    pub static ERRORBUFFER : c_int = OBJECTPOINT + 10;
    pub static WRITEFUNCTION : c_int = FUNCTIONPOINT + 11;
    pub static READFUNCTION : c_int = FUNCTIONPOINT + 12;
    pub static TIMEOUT : c_int = LONG + 13;
    pub static INFILESIZE : c_int = LONG + 14;
    pub static POSTFIELDS : c_int = OBJECTPOINT + 15;
    pub static REFERER : c_int = OBJECTPOINT + 16;
    pub static FTPPORT : c_int = OBJECTPOINT + 17;
    pub static USERAGENT : c_int = OBJECTPOINT + 18;
    pub static LOW_SPEED_LIMIT : c_int = LONG + 19;
    pub static LOW_SPEED_TIME : c_int = LONG + 20;
    pub static RESUME_FROM : c_int = LONG + 21;
    pub static COOKIE : c_int = OBJECTPOINT + 22;
    pub static HTTPHEADER : c_int = OBJECTPOINT + 23;
    pub static HTTPPOST : c_int = OBJECTPOINT + 24;
    pub static SSLCERT : c_int = OBJECTPOINT + 25;
    pub static KEYPASSWD : c_int = OBJECTPOINT + 26;
    pub static CRLF : c_int = LONG + 27;
    pub static QUOTE : c_int = OBJECTPOINT + 28;
    pub static WRITEHEADER : c_int = OBJECTPOINT + 29;
    pub static COOKIEFILE : c_int = OBJECTPOINT + 31;
    pub static SSLVERSION : c_int = LONG + 32;
    pub static TIMECONDITION : c_int = LONG + 33;
    pub static TIMEVALUE : c_int = LONG + 34;
    pub static CUSTOMREQUEST : c_int = OBJECTPOINT + 36;
    pub static STDERR : c_int = OBJECTPOINT + 37;
    pub static POSTQUOTE : c_int = OBJECTPOINT + 39;
    pub static WRITEINFO : c_int = OBJECTPOINT + 40;
    pub static VERBOSE : c_int = LONG + 41;
    pub static HEADER : c_int = LONG + 42;
    pub static NOPROGRESS : c_int = LONG + 43;
    pub static NOBODY : c_int = LONG + 44;
    pub static FAILONERROR : c_int = LONG + 45;
    pub static UPLOAD : c_int = LONG + 46;
    pub static POST : c_int = LONG + 47;
    pub static DIRLISTONLY : c_int = LONG + 48;
    pub static APPEND : c_int = LONG + 50;
    pub static NETRC : c_int = LONG + 51;
    pub static FOLLOWLOCATION : c_int = LONG + 52;
    pub static TRANSFERTEXT : c_int = LONG + 53;
    pub static PUT : c_int = LONG + 54;
    pub static PROGRESSFUNCTION : c_int = FUNCTIONPOINT + 56;
    pub static PROGRESSDATA : c_int = OBJECTPOINT + 57;
    pub static AUTOREFERER : c_int = LONG + 58;
    pub static PROXYPORT : c_int = LONG + 59;
    pub static POSTFIELDSIZE : c_int = LONG + 60;
    pub static HTTPPROXYTUNNEL : c_int = LONG + 61;
    pub static INTERFACE : c_int = OBJECTPOINT + 62;
    pub static KRBLEVEL : c_int = OBJECTPOINT + 63;
    pub static SSL_VERIFYPEER : c_int = LONG + 64;
    pub static CAINFO : c_int = OBJECTPOINT + 65;
    pub static MAXREDIRS : c_int = LONG + 68;
    pub static FILETIME : c_int = LONG + 69;
    pub static TELNETOPTIONS : c_int = OBJECTPOINT + 70;
    pub static MAXCONNECTS : c_int = LONG + 71;
    pub static CLOSEPOLICY : c_int = LONG + 72;
    pub static FRESH_CONNECT : c_int = LONG + 74;
    pub static FORBID_REUSE : c_int = LONG + 75;
    pub static RANDOM_FILE : c_int = OBJECTPOINT + 76;
    pub static EGDSOCKET : c_int = OBJECTPOINT + 77;
    pub static CONNECTTIMEOUT : c_int = LONG + 78;
    pub static HEADERFUNCTION : c_int = FUNCTIONPOINT + 79;
    pub static HTTPGET : c_int = LONG + 80;
    pub static SSL_VERIFYHOST : c_int = LONG + 81;
    pub static COOKIEJAR : c_int = OBJECTPOINT + 82;
    pub static SSL_CIPHER_LIST : c_int = OBJECTPOINT + 83;
    pub static HTTP_VERSION : c_int = LONG + 84;
    pub static FTP_USE_EPSV : c_int = LONG + 85;
    pub static SSLCERTTYPE : c_int = OBJECTPOINT + 86;
    pub static SSLKEY : c_int = OBJECTPOINT + 87;
    pub static SSLKEYTYPE : c_int = OBJECTPOINT + 88;
    pub static SSLENGINE : c_int = OBJECTPOINT + 89;
    pub static SSLENGINE_DEFAULT : c_int = LONG + 90;
    pub static DNS_USE_GLOBAL_CACHE : c_int = LONG + 91;
    pub static DNS_CACHE_TIMEOUT : c_int = LONG + 92;
    pub static PREQUOTE : c_int = OBJECTPOINT + 93;
    pub static DEBUGFUNCTION : c_int = FUNCTIONPOINT + 94;
    pub static DEBUGDATA : c_int = OBJECTPOINT + 95;
    pub static COOKIESESSION : c_int = LONG + 96;
    pub static CAPATH : c_int = OBJECTPOINT + 97;
    pub static BUFFERSIZE : c_int = LONG + 98;
    pub static NOSIGNAL : c_int = LONG + 99;
    pub static SHARE : c_int = OBJECTPOINT + 100;
    pub static PROXYTYPE : c_int = LONG + 101;
    pub static ACCEPT_ENCODING : c_int = OBJECTPOINT + 102;
    pub static PRIVATE : c_int = OBJECTPOINT + 103;
    pub static HTTP200ALIASES : c_int = OBJECTPOINT + 104;
    pub static UNRESTRICTED_AUTH : c_int = LONG + 105;
    pub static FTP_USE_EPRT : c_int = LONG + 106;
    pub static HTTPAUTH : c_int = LONG + 107;
    pub static SSL_CTX_FUNCTION : c_int = FUNCTIONPOINT + 108;
    pub static SSL_CTX_DATA : c_int = OBJECTPOINT + 109;
    pub static FTP_CREATE_MISSING_DIRS : c_int = LONG + 110;
    pub static PROXYAUTH : c_int = LONG + 111;
    pub static FTP_RESPONSE_TIMEOUT : c_int = LONG + 112;
    pub static SERVER_RESPONSE_TIMEOUT : c_int = LONG + 112;
    pub static IPRESOLVE : c_int = LONG + 113;
    pub static MAXFILESIZE : c_int = LONG + 114;
    pub static INFILESIZE_LARGE : c_int = OFF_T + 115;
    pub static RESUME_FROM_LARGE : c_int = OFF_T + 116;
    pub static MAXFILESIZE_LARGE : c_int = OFF_T + 117;
    pub static NETRC_FILE : c_int = OBJECTPOINT + 118;
    pub static USE_SSL : c_int = LONG + 119;
    pub static POSTFIELDSIZE_LARGE : c_int = OFF_T + 120;
    pub static TCP_NODELAY : c_int = LONG + 121;
    pub static FTPSSLAUTH : c_int = LONG + 129;
    pub static IOCTLFUNCTION : c_int = FUNCTIONPOINT + 130;
    pub static IOCTLDATA : c_int = OBJECTPOINT + 131;
    pub static FTP_ACCOUNT : c_int = OBJECTPOINT + 134;
    pub static COOKIELIST : c_int = OBJECTPOINT + 135;
    pub static IGNORE_CONTENT_LENGTH : c_int = LONG + 136;
    pub static FTP_SKIP_PASV_IP : c_int = LONG + 137;
    pub static FTP_FILEMETHOD : c_int = LONG + 138;
    pub static LOCALPORT : c_int = LONG + 139;
    pub static LOCALPORTRANGE : c_int = LONG + 140;
    pub static CONNECT_ONLY : c_int = LONG + 141;
    pub static CONV_FROM_NETWORK_FUNCTION : c_int = FUNCTIONPOINT + 142;
    pub static CONV_TO_NETWORK_FUNCTION : c_int = FUNCTIONPOINT + 143;
    pub static CONV_FROM_UTF8_FUNCTION : c_int = FUNCTIONPOINT + 144;
    pub static MAX_SEND_SPEED_LARGE : c_int = OFF_T + 145;
    pub static MAX_RECV_SPEED_LARGE : c_int = OFF_T + 146;
    pub static FTP_ALTERNATIVE_TO_USER : c_int = OBJECTPOINT + 147;
    pub static SOCKOPTFUNCTION : c_int = FUNCTIONPOINT + 148;
    pub static SOCKOPTDATA : c_int = OBJECTPOINT + 149;
    pub static SSL_SESSIONID_CACHE : c_int = LONG + 150;
    pub static SSH_AUTH_TYPES : c_int = LONG + 151;
    pub static SSH_PUBLIC_KEYFILE : c_int = OBJECTPOINT + 152;
    pub static SSH_PRIVATE_KEYFILE : c_int = OBJECTPOINT + 153;
    pub static FTP_SSL_CCC : c_int = LONG + 154;
    pub static TIMEOUT_MS : c_int = LONG + 155;
    pub static CONNECTTIMEOUT_MS : c_int = LONG + 156;
    pub static HTTP_TRANSFER_DECODING : c_int = LONG + 157;
    pub static HTTP_CONTENT_DECODING : c_int = LONG + 158;
    pub static NEW_FILE_PERMS : c_int = LONG + 159;
    pub static NEW_DIRECTORY_PERMS : c_int = LONG + 160;
    pub static POSTREDIR : c_int = LONG + 161;
    pub static SSH_HOST_PUBLIC_KEY_MD5 : c_int = OBJECTPOINT + 162;
    pub static OPENSOCKETFUNCTION : c_int = FUNCTIONPOINT + 163;
    pub static OPENSOCKETDATA : c_int = OBJECTPOINT + 164;
    pub static COPYPOSTFIELDS : c_int = OBJECTPOINT + 165;
    pub static PROXY_TRANSFER_MODE : c_int = LONG + 166;
    pub static SEEKFUNCTION : c_int = FUNCTIONPOINT + 167;
    pub static SEEKDATA : c_int = OBJECTPOINT + 168;
    pub static CRLFILE : c_int = OBJECTPOINT + 169;
    pub static ISSUERCERT : c_int = OBJECTPOINT + 170;
    pub static ADDRESS_SCOPE : c_int = LONG + 171;
    pub static CERTINFO : c_int = LONG + 172;
    pub static USERNAME : c_int = OBJECTPOINT + 173;
    pub static PASSWORD : c_int = OBJECTPOINT + 174;
    pub static PROXYUSERNAME : c_int = OBJECTPOINT + 175;
    pub static PROXYPASSWORD : c_int = OBJECTPOINT + 176;
    pub static NOPROXY : c_int = OBJECTPOINT + 177;
    pub static TFTP_BLKSIZE : c_int = LONG + 178;
    pub static SOCKS5_GSSAPI_SERVICE : c_int = OBJECTPOINT + 179;
    pub static SOCKS5_GSSAPI_NEC : c_int = LONG + 180;
    pub static PROTOCOLS : c_int = LONG + 181;
    pub static REDIR_PROTOCOLS : c_int = LONG + 182;
    pub static SSH_KNOWNHOSTS : c_int = OBJECTPOINT + 183;
    pub static SSH_KEYFUNCTION : c_int = FUNCTIONPOINT + 184;
    pub static SSH_KEYDATA : c_int = OBJECTPOINT + 185;
    pub static MAIL_FROM : c_int = OBJECTPOINT + 186;
    pub static MAIL_RCPT : c_int = OBJECTPOINT + 187;
    pub static FTP_USE_PRET : c_int = LONG + 188;
    pub static RTSP_REQUEST : c_int = LONG + 189;
    pub static RTSP_SESSION_ID : c_int = OBJECTPOINT + 190;
    pub static RTSP_STREAM_URI : c_int = OBJECTPOINT + 191;
    pub static RTSP_TRANSPORT : c_int = OBJECTPOINT + 192;
    pub static RTSP_CLIENT_CSEQ : c_int = LONG + 193;
    pub static RTSP_SERVER_CSEQ : c_int = LONG + 194;
    pub static INTERLEAVEDATA : c_int = OBJECTPOINT + 195;
    pub static INTERLEAVEFUNCTION : c_int = FUNCTIONPOINT + 196;
    pub static WILDCARDMATCH : c_int = LONG + 197;
    pub static CHUNK_BGN_FUNCTION : c_int = FUNCTIONPOINT + 198;
    pub static CHUNK_END_FUNCTION : c_int = FUNCTIONPOINT + 199;
    pub static FNMATCH_FUNCTION : c_int = FUNCTIONPOINT + 200;
    pub static CHUNK_DATA : c_int = OBJECTPOINT + 201;
    pub static FNMATCH_DATA : c_int = OBJECTPOINT + 202;
    pub static RESOLVE : c_int = OBJECTPOINT + 203;
    pub static TLSAUTH_USERNAME : c_int = OBJECTPOINT + 204;
    pub static TLSAUTH_PASSWORD : c_int = OBJECTPOINT + 205;
    pub static TLSAUTH_TYPE : c_int = OBJECTPOINT + 206;
    pub static TRANSFER_ENCODING : c_int = LONG + 207;
    pub static CLOSESOCKETFUNCTION : c_int = FUNCTIONPOINT + 208;
    pub static CLOSESOCKETDATA : c_int = OBJECTPOINT + 209;
    pub static GSSAPI_DELEGATION : c_int = LONG + 210;
    pub static DNS_SERVERS : c_int = OBJECTPOINT + 211;
    pub static ACCEPTTIMEOUT_MS : c_int = LONG + 212;
    pub static TCP_KEEPALIVE : c_int = LONG + 213;
    pub static TCP_KEEPIDLE : c_int = LONG + 214;
    pub static TCP_KEEPINTVL : c_int = LONG + 215;
    pub static SSL_OPTIONS : c_int = LONG + 216;
    pub static MAIL_AUTH : c_int = OBJECTPOINT + 217;
}
