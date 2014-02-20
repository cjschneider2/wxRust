use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct wxMediaCtrl { ptr: *mut c_void }
impl _wxMediaCtrl for wxMediaCtrl {}
impl _wxWindow for wxMediaCtrl {}
impl _wxEvtHandler for wxMediaCtrl {}
impl _wxObject for wxMediaCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMediaCtrl {
    pub fn from(ptr: *mut c_void) -> wxMediaCtrl { wxMediaCtrl { ptr: ptr } }
    pub fn null() -> wxMediaCtrl { wxMediaCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(parent: &T, windowID: c_int, fileName: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: &str, name: &str) -> wxMediaCtrl {
        let fileName = wxT(fileName);
        let szBackend = wxT(szBackend);
        let name = wxT(name);
        unsafe { wxMediaCtrl { ptr: wxMediaCtrl_Create(parent.ptr(), windowID, fileName.ptr(), x, y, w, h, style, szBackend.ptr(), name.ptr()) } }
    }
}

pub trait _wxMediaCtrl : _wxWindow {
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self.ptr()) }
    }
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self.ptr()) }
    }
    fn getState(&self) -> c_int {
        unsafe { wxMediaCtrl_GetState(self.ptr()) }
    }
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self.ptr()) }
    }
    fn load(&self, fileName: &str) -> c_int {
        let fileName = wxT(fileName);
        unsafe { wxMediaCtrl_Load(self.ptr(), fileName.ptr()) }
    }
    fn loadURI(&self, uri: &str) -> c_int {
        let uri = wxT(uri);
        unsafe { wxMediaCtrl_LoadURI(self.ptr(), uri.ptr()) }
    }
    fn loadURIWithProxy(&self, uri: &str, proxy: &str) -> c_int {
        let uri = wxT(uri);
        let proxy = wxT(proxy);
        unsafe { wxMediaCtrl_LoadURIWithProxy(self.ptr(), uri.ptr(), proxy.ptr()) }
    }
    fn pause(&self) -> c_int {
        unsafe { wxMediaCtrl_Pause(self.ptr()) }
    }
    fn play(&self) -> c_int {
        unsafe { wxMediaCtrl_Play(self.ptr()) }
    }
    fn seek(&self, offsetWhere: i64, mode: c_int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self.ptr(), offsetWhere, mode) }
    }
    fn setPlaybackRate(&self, dRate: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetPlaybackRate(self.ptr(), dRate) }
    }
    fn setVolume(&self, dVolume: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetVolume(self.ptr(), dVolume) }
    }
    fn showPlayerControls(&self, flags: c_int) -> c_int {
        unsafe { wxMediaCtrl_ShowPlayerControls(self.ptr(), flags) }
    }
    fn stop(&self) -> c_int {
        unsafe { wxMediaCtrl_Stop(self.ptr()) }
    }
    fn tell(&self) -> i64 {
        unsafe { wxMediaCtrl_Tell(self.ptr()) }
    }
}

pub struct wxMediaEvent { ptr: *mut c_void }
impl _wxMediaEvent for wxMediaEvent {}
impl _wxNotifyEvent for wxMediaEvent {}
impl _wxCommandEvent for wxMediaEvent {}
impl _wxEvent for wxMediaEvent {}
impl _wxObject for wxMediaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMediaEvent {
    pub fn from(ptr: *mut c_void) -> wxMediaEvent { wxMediaEvent { ptr: ptr } }
    pub fn null() -> wxMediaEvent { wxMediaEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMediaEvent : _wxNotifyEvent {
}

