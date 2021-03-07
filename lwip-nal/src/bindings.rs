#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern "C" {
    pub static ip_addr_any: ip_addr_t;

    pub fn uxQueueMessagesWaiting(xQueue: QueueHandle_t) -> c_ulong;
    pub fn LwIP_Init();
    pub fn LwIP_ReleaseIP(idx: u8);
    pub fn LwIP_DHCP(idx: u8, dhcp_state: u8) -> u8;
    pub fn LwIP_GetIP_Idx(idx: u8) -> *mut c_uchar;

    pub fn pbuf_ref(p: *mut pbuf);
    pub fn pbuf_free(p: *mut pbuf) -> u8;
    pub fn pbuf_chain(head: *mut pbuf, tail: *mut pbuf);
    pub fn pbuf_dechain(p: *mut pbuf) -> *mut pbuf;

    pub fn netconn_delete(conn: *mut netconn) -> err_t;
    pub fn netconn_bind(conn: *mut netconn, addr: *const ip_addr_t, port: u16) -> err_t;
    pub fn netconn_connect(conn: *mut netconn, addr: *const ip_addr_t, port: u16) -> err_t;
    pub fn netconn_disconnect(conn: *mut netconn) -> err_t;
    pub fn netconn_listen_with_backlog(conn: *mut netconn, backlog: u8) -> err_t;
    pub fn netconn_accept(conn: *mut netconn, new_conn: *mut *mut netconn) -> err_t;
    pub fn netconn_recv_tcp_pbuf(conn: *mut netconn, new_buf: *mut *mut pbuf) -> err_t;
    pub fn netconn_close(conn: *mut netconn) -> err_t;
    pub fn netconn_shutdown(conn: *mut netconn, shut_rx: u8, shut_tx: u8) -> err_t;
    pub fn netconn_abort(conn: *mut netconn) -> err_t;
    pub fn netconn_gethostbyname(name: *const c_char, addr: *mut ip_addr_t) -> err_t;
    pub fn netconn_new_with_proto_and_callback(
        t: netconn_type,
        proto: u8,
        callback: netconn_callback,
    ) -> *mut netconn;
    pub fn netconn_getaddr(
        conn: *mut netconn,
        addr: *mut ip_addr_t,
        port: *mut u16,
        local: u8,
    ) -> err_t;
    pub fn netconn_write_partly(
        conn: *mut netconn,
        dataptr: *const c_void,
        size: size_t,
        apiflags: u8,
        bytes_written: *mut size_t,
    ) -> err_t;
}

pub type c_void = core::ffi::c_void;
pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type size_t = c_ulong;

pub type ip4_addr_t = ip4_addr;
pub type ip_addr_t = ip4_addr_t;

pub type QueueHandle_t = *mut QueueDefinition;
pub type SemaphoreHandle_t = QueueHandle_t;

pub type sys_sem_t = SemaphoreHandle_t;
pub type sys_mutex_t = SemaphoreHandle_t;
pub type sys_mbox_t = QueueHandle_t;

pub type err_t = i8;
pub type err_enum_t = err_t;

pub type netconn_callback =
    ::core::option::Option<unsafe extern "C" fn(conn: *mut netconn, evt: netconn_evt, len: u16)>;

pub type netconn_type = u8;
pub type netconn_state = u8;
pub type netconn_evt = u8;

pub type tcpwnd_size_t = u16;
pub type tcpflags_t = u8;
pub type tcp_state = u8;

pub const IPPROTO_IP: u8 = 0;
pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_UDPLITE: u8 = 136;
pub const IPPROTO_RAW: u8 = 255;

pub const ERR_OK: err_enum_t = 0;
pub const ERR_MEM: err_enum_t = -1;
pub const ERR_BUF: err_enum_t = -2;
pub const ERR_TIMEOUT: err_enum_t = -3;
pub const ERR_RTE: err_enum_t = -4;
pub const ERR_INPROGRESS: err_enum_t = -5;
pub const ERR_VAL: err_enum_t = -6;
pub const ERR_WOULDBLOCK: err_enum_t = -7;
pub const ERR_USE: err_enum_t = -8;
pub const ERR_ALREADY: err_enum_t = -9;
pub const ERR_ISCONN: err_enum_t = -10;
pub const ERR_CONN: err_enum_t = -11;
pub const ERR_IF: err_enum_t = -12;
pub const ERR_ABRT: err_enum_t = -13;
pub const ERR_RST: err_enum_t = -14;
pub const ERR_CLSD: err_enum_t = -15;
pub const ERR_ARG: err_enum_t = -16;

pub const DHCP_START: u8 = 0;
pub const DHCP_WAIT_ADDRESS: u8 = 1;
pub const DHCP_ADDRESS_ASSIGNED: u8 = 2;
pub const DHCP_RELEASE_IP: u8 = 3;
pub const DHCP_STOP: u8 = 4;
pub const DHCP_TIMEOUT: u8 = 5;

pub const NETCONN_TYPE_INVALID: netconn_type = 0;
pub const NETCONN_TYPE_TCP: netconn_type = 16;
pub const NETCONN_TYPE_UDP: netconn_type = 32;
pub const NETCONN_TYPE_UDPLITE: netconn_type = 33;
pub const NETCONN_TYPE_UDPNOCHKSUM: netconn_type = 34;
pub const NETCONN_TYPE_RAW: netconn_type = 64;

pub const NETCONN_STATE_NONE: netconn_state = 0;
pub const NETCONN_STATE_WRITE: netconn_state = 1;
pub const NETCONN_STATE_LISTEN: netconn_state = 2;
pub const NETCONN_STATE_CONNECT: netconn_state = 3;
pub const NETCONN_STATE_CLOSE: netconn_state = 4;

pub const NETCONN_EVT_EVT_RCVPLUS: netconn_evt = 0;
pub const NETCONN_EVT_EVT_RCVMINUS: netconn_evt = 1;
pub const NETCONN_EVT_EVT_SENDPLUS: netconn_evt = 2;
pub const NETCONN_EVT_EVT_SENDMINUS: netconn_evt = 3;
pub const NETCONN_EVT_EVT_ERROR: netconn_evt = 4;

pub const NETCONN_NOFLAG: u8 = 0;
pub const NETCONN_NOCOPY: u8 = 0;
pub const NETCONN_COPY: u8 = 1;
pub const NETCONN_MORE: u8 = 2;
pub const NETCONN_DONTBLOCK: u8 = 4;
pub const NETCONN_FLAG_NON_BLOCKING: u8 = 2;
pub const NETCONN_FLAG_IN_NONBLOCKING_CONNECT: u8 = 4;
pub const NETCONN_FLAG_CHECK_WRITESPACE: u8 = 16;
pub const NETCONN_DNS_IPV4: u8 = 0;
pub const NETCONN_DNS_IPV6: u8 = 1;
pub const NETCONN_DNS_IPV4_IPV6: u8 = 2;
pub const NETCONN_DNS_IPV6_IPV4: u8 = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pbuf {
    pub next: *mut pbuf,
    pub payload: *mut c_void,
    pub tot_len: u16,
    pub len: u16,
    pub type_: u8,
    pub flags: u8,
    pub ref_: u16,
}
impl Default for pbuf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ip4_addr {
    pub addr: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QueueDefinition {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pcb {
    pub tcp: *mut tcp_pcb,
    pub tcp_listen: *mut tcp_pcb_listen,
    _bindgen_union_align: usize,
}
impl Default for pcb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for pcb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{{PCB}}")
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct netconn {
    pub type_: netconn_type,
    pub state: netconn_state,
    pub pcb: pcb,
    pub last_err: err_t,
    pub op_completed: sys_sem_t,
    pub recvmbox: sys_mbox_t,
    pub acceptmbox: sys_mbox_t,
    pub socket: c_int,
    pub recv_timeout: c_int,
    pub flags: u8,
    pub write_offset: size_t,
    pub current_msg: *mut c_void,
    pub callback: netconn_callback,
}

impl Default for netconn {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_pcb_listen {
    pub local_ip: ip_addr_t,
    pub remote_ip: ip_addr_t,
    pub so_options: u8,
    pub tos: u8,
    pub ttl: u8,
    pub next: *mut tcp_pcb_listen,
    pub callback_arg: *mut c_void,
    pub state: tcp_state,
    pub prio: u8,
    pub local_port: u16,
    pub accept: *const c_void,
}

impl Default for tcp_pcb_listen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_pcb {
    pub local_ip: ip_addr_t,
    pub remote_ip: ip_addr_t,
    pub so_options: u8,
    pub tos: u8,
    pub ttl: u8,
    pub next: *mut tcp_pcb,
    pub callback_arg: *mut c_void,
    pub state: tcp_state,
    pub prio: u8,
    pub local_port: u16,
    pub remote_port: u16,
    pub flags: tcpflags_t,
    pub polltmr: u8,
    pub pollinterval: u8,
    pub last_timer: u8,
    pub tmr: u32,
    pub rcv_nxt: u32,
    pub rcv_wnd: tcpwnd_size_t,
    pub rcv_ann_wnd: tcpwnd_size_t,
    pub rcv_ann_right_edge: u32,
    pub rtime: c_short,
    pub mss: u16,
    pub rttest: u32,
    pub rtseq: u32,
    pub sa: c_short,
    pub sv: c_short,
    pub rto: c_short,
    pub nrtx: u8,
    pub dupacks: u8,
    pub lastack: u32,
    pub cwnd: tcpwnd_size_t,
    pub ssthresh: tcpwnd_size_t,
    pub snd_nxt: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub snd_lbb: u32,
    pub snd_wnd: tcpwnd_size_t,
    pub snd_wnd_max: tcpwnd_size_t,
    pub snd_buf: tcpwnd_size_t,
    pub snd_queuelen: u16,
    pub unsent_oversize: u16,
    pub unsent: *mut c_void,
    pub unacked: *mut c_void,
    pub ooseq: *mut c_void,
    pub refused_data: *mut pbuf,
    pub listener: *mut tcp_pcb_listen,
    pub sent: *const c_void,
    pub recv: *const c_void,
    pub connected: *const c_void,
    pub poll: *const c_void,
    pub errf: *const c_void,
    pub keep_idle: u32,
    pub keep_intvl: u32,
    pub keep_cnt: u32,
    pub persist_cnt: u8,
    pub persist_backoff: u8,
    pub keep_cnt_sent: u8,
}

impl Default for tcp_pcb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
