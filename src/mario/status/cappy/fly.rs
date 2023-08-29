use crate::imports::imports_agent::*;
use super::*;
pub const NEXT_STATUS: i32 = CAPTOSS_STATUS_KIND_HOLD;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        println!("Isabelle fly?");
        return 0.into();
    }
    let owner_boma = get_owner_boma(weapon);
    let stick_y = ControlModule::get_stick_y(owner_boma);
    let stick_y_mul = if stick_y.abs() < 0.2 {0.0} else {WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity"))};

    let lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, lr);
    PostureModule::set_scale(weapon.module_accessor, 1.375,false);
    
    //let roty = if lr > 0.0 {0.0} else {180.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:0.0,z:0.0}, 0);

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));

    let y_speed = if stick_y.abs() < 0.2 {0.0} else {speed_max*stick_y*stick_y_mul};
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_max*lr,
        y_speed
    );
    let y_max = if stick_y.abs() < 0.2 {0.0} else {speed_max*stick_y_mul};
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_max,
        y_max
    );
    let y_accel = if stick_y.abs() < 0.2 {0.0} else {-accel*stick_y.signum()*stick_y_mul};
    println!("FLY: Init (YSpeed: {y_speed} YMax: {y_max} Y_Accel: {y_accel})");
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -accel*lr,
        y_accel
    );
    /*
    let speed_rot = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("rot_speed"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        speed_rot*lr
    ); */

    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ROT_Y_LR | *FS_SUCCEEDS_KEEP_EFFECT,
    );
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(captoss_fly_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(captoss_fly_main_status_loop as *const () as _))
}
unsafe extern "C" fn captoss_fly_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn captoss_fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_min_mul = speed_min*1.0;
    
    let sum_speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let sum_speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP)
    {
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if sum_speed_len <= speed_min_mul {
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
            //weapon.change_status(CAPTOSS_STATUS_KIND_HOP.into(),false.into());
            StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
            return 0.into();
        }
    }
    else{
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK);
    }
    /* 
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT){
        WorkModule::on_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    }
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION) {
        if StopModule::is_stop(weapon.module_accessor){
            let can_penetrate = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("is_penetration"))==0;
            if !can_penetrate {
                println!("Hit!");
                //StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
                //return 0.into();
            }
        }
    }*/
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR){
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
        //StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    {
        StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
    }
    else if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_DOWN) as u32)
    {
        let bound_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("floor_bound_x_mul"));
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.5, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        //KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.5, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        //KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: 1.0, y: bound_mul, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    }

    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    captoss_fly_init::install();
    captoss_fly_pre::install();
    captoss_fly_main::install();
    captoss_fly_exec::install();
    captoss_fly_end::install();

}
