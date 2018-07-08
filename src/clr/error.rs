//std
use std::error::Error;
use std::fmt;

//3rd party
use winapi::shared::winerror::HRESULT;

//self

#[derive(Copy, Clone)]
pub enum ErrorSource {
    CallAppDomain(HRESULT),
    RustAppDomain,
    SafeArray(HRESULT),
    CAppDomainManager(HRESULT),
}

use self::ErrorSource::*;

impl fmt::Debug for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            CallAppDomain(_) => "C/C++/CLR - AppDomain",
            RustAppDomain => "Rust - App Domain",
            SafeArray(_) => "C/C++/CLR - SafeArray", 
            CAppDomainManager(_) => "C/C++/CLR - AppDomainManager"
        }; 

        let printable = match *self {
            CallAppDomain(hr) | SafeArray(hr) | CAppDomainManager(hr) => format!("{}(0x{:x})", printable, hr), 
            RustAppDomain => String::from(printable)
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
pub enum ClrErrorKind {
    NulPointer(ErrorSource), 
    InnerCall(ErrorSource), 
    InvalidState(ErrorSource),
}
use self::ClrErrorKind::*;


#[derive(Debug)]
pub struct InnerClrError {
    hresult: Option<HRESULT>,
}

impl InnerClrError {
    pub fn new(hr: HRESULT) -> InnerClrError {
        InnerClrError { hresult: Some(hr) }
    }
}

impl Error for InnerClrError {
    fn description(&self) -> &str {
        "Error happened within CLR/C++ hosting code"
    }

    //no inner cause
}

impl fmt::Display for InnerClrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HRESULT: {:x}", self.hresult.unwrap_or(0))
    }
}

#[derive(Debug)]
pub struct ClrError {
    inner: Option<InnerClrError>, 
    kind: ClrErrorKind
}

impl ClrError {
    pub fn nul(source: ErrorSource) -> ClrError {
        ClrError { inner: None, kind: ClrErrorKind::NulPointer(source) }
    }

    pub fn inner_call(source: ErrorSource) -> ClrError {
        match &source {
            CallAppDomain(hr) | SafeArray(hr) | CAppDomainManager(hr) => ClrError {inner: Some(InnerClrError::new(*hr)), kind: InnerCall(source) }, 
            RustAppDomain => ClrError::nul(source), 
        }
    }

    pub fn invalid(source: ErrorSource) -> ClrError {
        ClrError {inner: None, kind: InvalidState(source)}
    }
}

impl fmt::Display for ClrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Error for ClrError {
    fn description(&self) -> &str {
        "Error occurred within CLR/C++ hosting code."
    }

    fn cause(&self) -> Option<&Error> {
        match &self.inner {
            Some(inn) => Some(inn), 
            None => None
        }
    }
}
