/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sslcContext {
    pub servhandle: Handle,
    pub sslchandle: u32_,
    pub sharedmem_handle: Handle,
}
impl ::core::default::Default for sslcContext {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SSLC_DefaultRootCert {
    SSLC_DefaultRootCert_Nintendo_CA = 1,
    SSLC_DefaultRootCert_Nintendo_CA_G2 = 2,
    SSLC_DefaultRootCert_Nintendo_CA_G3 = 3,
    SSLC_DefaultRootCert_Nintendo_Class2_CA = 4,
    SSLC_DefaultRootCert_Nintendo_Class2_CA_G2 = 5,
    SSLC_DefaultRootCert_Nintendo_Class2_CA_G3 = 6,
    SSLC_DefaultRootCert_CyberTrust = 7,
    SSLC_DefaultRootCert_AddTrust_External_CA = 8,
    SSLC_DefaultRootCert_COMODO = 9,
    SSLC_DefaultRootCert_USERTrust = 10,
    SSLC_DefaultRootCert_DigiCert_EV = 11,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SSLC_DefaultClientCert { SSLC_DefaultClientCert_ClCertA = 64, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed1 {
    SSLCOPT_Default = 0,
    SSLCOPT_DisableVerify = 512,
    SSLCOPT_TLSv10 = 2048,
}
extern "C" {
    pub fn sslcInit(session_handle: Handle) -> Result;
    pub fn sslcExit();
    pub fn sslcCreateRootCertChain(RootCertChain_contexthandle: *mut u32_)
     -> Result;
    pub fn sslcDestroyRootCertChain(RootCertChain_contexthandle: u32_)
     -> Result;
    pub fn sslcAddTrustedRootCA(RootCertChain_contexthandle: u32_,
                                cert: *const u8_, certsize: u32_,
                                cert_contexthandle: *mut u32_) -> Result;
    pub fn sslcRootCertChainAddDefaultCert(RootCertChain_contexthandle: u32_,
                                           certID: SSLC_DefaultRootCert,
                                           cert_contexthandle: *mut u32_)
     -> Result;
    pub fn sslcRootCertChainRemoveCert(RootCertChain_contexthandle: u32_,
                                       cert_contexthandle: u32_) -> Result;
    pub fn sslcCreate8CertChain(CertChain_contexthandle: *mut u32_) -> Result;
    pub fn sslcDestroy8CertChain(CertChain_contexthandle: u32_) -> Result;
    pub fn sslc8CertChainAddCert(CertChain_contexthandle: u32_,
                                 cert: *const u8_, certsize: u32_,
                                 cert_contexthandle: *mut u32_) -> Result;
    pub fn sslc8CertChainAddDefaultCert(CertChain_contexthandle: u32_,
                                        certID: u8_,
                                        cert_contexthandle: *mut u32_)
     -> Result;
    pub fn sslc8CertChainRemoveCert(CertChain_contexthandle: u32_,
                                    cert_contexthandle: u32_) -> Result;
    pub fn sslcOpenClientCertContext(cert: *const u8_, certsize: u32_,
                                     key: *const u8_, keysize: u32_,
                                     ClientCert_contexthandle: *mut u32_)
     -> Result;
    pub fn sslcOpenDefaultClientCertContext(certID: SSLC_DefaultClientCert,
                                            ClientCert_contexthandle:
                                                *mut u32_) -> Result;
    pub fn sslcCloseClientCertContext(ClientCert_contexthandle: u32_)
     -> Result;
    pub fn sslcSeedRNG() -> Result;
    pub fn sslcGenerateRandomData(buf: *mut u8_, size: u32_) -> Result;
    pub fn sslcCreateContext(context: *mut sslcContext, sockfd: ::libc::c_int,
                             input_opt: u32_, hostname: *const ::libc::c_char)
     -> Result;
    pub fn sslcDestroyContext(context: *mut sslcContext) -> Result;
    pub fn sslcStartConnection(context: *mut sslcContext,
                               internal_retval: *mut ::libc::c_int,
                               out: *mut u32_) -> Result;
    pub fn sslcRead(context: *mut sslcContext, buf: *mut ::libc::c_void,
                    len: ::libc::size_t, peek: u8) -> Result;
    pub fn sslcWrite(context: *mut sslcContext, buf: *const ::libc::c_void,
                     len: ::libc::size_t) -> Result;
    pub fn sslcContextSetRootCertChain(context: *mut sslcContext,
                                       handle: u32_) -> Result;
    pub fn sslcContextSetClientCert(context: *mut sslcContext, handle: u32_)
     -> Result;
    pub fn sslcContextSetHandle8(context: *mut sslcContext, handle: u32_)
     -> Result;
    pub fn sslcContextClearOpt(context: *mut sslcContext, bitmask: u32_)
     -> Result;
    pub fn sslcContextGetProtocolCipher(context: *mut sslcContext,
                                        outprotocols: *mut ::libc::c_char,
                                        outprotocols_maxsize: u32_,
                                        outcipher: *mut ::libc::c_char,
                                        outcipher_maxsize: u32_) -> Result;
    pub fn sslcContextGetState(context: *mut sslcContext, out: *mut u32_)
     -> Result;
    pub fn sslcContextInitSharedmem(context: *mut sslcContext, buf: *mut u8_,
                                    size: u32_) -> Result;
    pub fn sslcAddCert(context: *mut sslcContext, buf: *const u8_, size: u32_)
     -> Result;
}
use ::types::*;