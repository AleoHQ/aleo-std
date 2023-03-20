// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the aleo-std library.

// The aleo-std library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The aleo-std library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the aleo-std library. If not, see <https://www.gnu.org/licenses/>.

#[cfg(feature = "cpu")]
pub use aleo_std_cpu::{get_cpu, Cpu};
pub use aleo_std_profiler::*;
#[cfg(feature = "storage")]
pub use aleo_std_storage::*;

pub mod prelude {
    #[cfg(feature = "cpu")]
    pub use aleo_std_cpu::{get_cpu, Cpu};
    pub use aleo_std_profiler::*;
    #[cfg(feature = "storage")]
    pub use aleo_std_storage::*;
    pub use aleo_std_time::time;
    pub use aleo_std_timed::timed;
    pub use aleo_std_timer::{finish, lap, timer};
}
