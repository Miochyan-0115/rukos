/* Copyright (c) [2023] [Syswonder Community]
 *   [Rukos] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

//! I/O multiplexing:
//!
//! * [`select`](select::sys_select)
//! * [`epoll_create`](epoll::sys_epoll_create)
//! * [`epoll_ctl`](epoll::sys_epoll_ctl)
//! * [`epoll_wait`](epoll::sys_epoll_wait)

#[cfg(feature = "epoll")]
mod epoll;
#[cfg(feature = "select")]
mod select;

#[cfg(feature = "epoll")]
pub use self::epoll::{sys_epoll_create, sys_epoll_ctl, sys_epoll_wait};
#[cfg(feature = "select")]
pub use self::select::sys_select;
