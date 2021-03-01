#![no_std]

extern crate alloc;

#[macro_use]
extern crate num_derive;

use alloc::string::String;
use alloc::vec;
use rtl8720_sys as sys;

mod access_point;
mod client;
mod common;
mod sections;
mod types;

pub use access_point::*;
pub use client::*;
pub use common::*;
use sections::*;
pub use types::*;

// Section from realtek headers for reference...

// int rltk_wlan_init(int idx_wlan, rtw_mode_t mode);				//return 0: success. -1:fail
// void rltk_wlan_deinit(void);
// void rltk_wlan_deinit_fastly(void);
// int rltk_wlan_start(int idx_wlan);
// void rltk_wlan_statistic(unsigned char idx);
// unsigned char rltk_wlan_running(unsigned char idx);		// interface is up. 0: interface is down
// int rltk_wlan_control(unsigned long cmd, void *data);
// int rltk_wlan_handshake_done(void);
// int rltk_wlan_rf_on(void);
// int rltk_wlan_rf_off(void);
// int rltk_wlan_check_bus(void);
// int rltk_wlan_wireless_mode(unsigned char mode);
// int rltk_wlan_get_wireless_mode(unsigned char *pmode);
// int rltk_wlan_set_wps_phase(unsigned char is_trigger_wps);
// int rtw_ps_enable(int enable);
// int rltk_wlan_is_connected_to_ap(void);
// void rltk_wlan_btcoex_set_bt_state(unsigned char state);
// void rltk_wlan_set_scan_chan_interval(unsigned short interval_ms);
// int rltk_wlan_change_channel_plan(unsigned char channel_plan);

// void rltk_wlan_enable_adaptivity(unsigned char enable);
// void rltk_wlan_set_adaptivity_mode(unsigned char mode);
// void rltk_wlan_enable_trp_tis_cert(unsigned char enable);
// void rltk_wlan_set_tx_pwr_lmt(unsigned char value);
// void rltk_wlan_set_tx_pwr_by_rate(unsigned char value);
// #ifdef CONFIG_POWER_SAVING
// void rltk_wlan_enable_powersave(unsigned char enable);
// #endif

// #ifdef CONFIG_INCLUDE_WPA_PSK
// void rltk_psk_essid_set(unsigned char index, int value);
// void rltk_psk_essid_strncpy_to(unsigned char index, char const * src, unsigned int length);
// void rltk_psk_essid_memcpy_to(unsigned char index, unsigned char * src);
// void rltk_psk_essid_memcpy_from(unsigned char index, unsigned char * dst, unsigned int length);
// void rltk_psk_essid_strcpy(unsigned char index, unsigned char * dst);
// unsigned int rltk_psk_essid_strlen(void);
// void rltk_psk_passphrase_set(unsigned char index, int value);
// void rltk_psk_passphrase_memcpy_to(unsigned char index, unsigned char * src);
// void rltk_psk_passphrase_memcpy_from(unsigned char index, unsigned char * dst, unsigned int length);
// unsigned char* rltk_psk_passphrase_get(void);
// unsigned int rltk_psk_passphrase_strlen(void);
// void rltk_wpa_global_PSK_set(unsigned char index, int value);
// void rltk_psk_passphrase64_memcpy_from(unsigned char * dst, unsigned int length);
// unsigned int rltk_psk_passphrase64_strlen(void);
// void rltk_wpa_global_PSK_memcpy_from(unsigned char index, unsigned char * dst, unsigned int length);
// void rltk_wpa_global_PSK_memcpy_to(unsigned char index, unsigned char * src);
// #endif

// #ifdef CONFIG_IEEE80211W
// void rltk_wlan_tx_sa_query(unsigned char key_type);
// void rltk_wlan_tx_deauth(unsigned char b_broadcast, unsigned char key_type);
// void rltk_wlan_tx_auth(void);
// #endif
