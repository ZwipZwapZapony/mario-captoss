pub mod imports_acmd {
    pub use {
        smash::{
            lib::{
                lua_const::*
            },
            app::{
                *,
                self,
                sv_animcmd::{
                    frame,
                    wait
                },
                lua_bind::*
            },
            hash40,
            lua2cpp::*,
            phx::*
        },
        smash_script::{
            *,
            macros::*
        },
        smashline::*,
        crate::vars::*,
        //crate::util::*,
        //crate::data::gamemode::*, 
        sharpsmashlinesuite::{
            *,
            util::{
                *,
                self
            },
            ext::*,
            getvar::*
        },
    };
}

pub mod imports_agent {
    pub use {
        smash::{
            lib::{
                L2CValue,
                L2CAgent,
                lua_const::*
            },
            app::{
                *,
                self,
                lua_bind::*,
            },
            hash40,
            lua2cpp::*,
            phx::*
        },
        smash_script::{
            *,
            macros::*
        },
        smashline::{
            *,
            Init,
            Pre,
            Main,
            Exec,
            End,
        },
        crate::vars::*,
        //crate::util::*,
        //crate::data::gamemode::*,
        sharpsmashlinesuite::{
            *,
            util::{
                *,
                self
            },
            ext::*,
            getvar::*
        },
    };
}
pub mod imports_status {
    pub use {
        crate::imports::imports_agent::*,
        smashline::{
            *,
            Init,
            Pre,
            Main,
            Exec,
            End,
        },
    };
}