use cxx::{type_id, ExternType};
use static_assertions::const_assert_eq;
use std::fmt::{self, Debug, Formatter};
use std::pin::Pin;

use crate::detour::{DtNavMesh, DtPolyRef, DtQueryFilter};
use crate::detour_crowd::local_boundary::DtLocalBoundary;
use crate::detour_crowd::obstacle_avoidance::DtObstacleAvoidanceParams;
use crate::detour_crowd::path_corridor::DtPathCorridor;

pub const DT_CROWDAGENT_MAX_NEIGHBOURS: i32 = 6;
pub const DT_CROWDAGENT_MAX_CORNERS: i32 = 4;
pub const DT_CROWD_MAX_OBSTAVOIDANCE_PARAMS: i32 = 8;
pub const DT_CROWD_MAX_QUERY_FILTER_TYPE: i32 = 16;

#[allow(dead_code)]
#[cxx::bridge]
pub(crate) mod ffi {
    #[repr(u32)]
    pub enum CrowdAgentState {
        DT_CROWDAGENT_STATE_INVALID,
        DT_CROWDAGENT_STATE_WALKING,
        DT_CROWDAGENT_STATE_OFFMESH,
    }

    #[repr(u32)]
    pub enum MoveRequestState {
        DT_CROWDAGENT_TARGET_NONE = 0,
        DT_CROWDAGENT_TARGET_FAILED,
        DT_CROWDAGENT_TARGET_VALID,
        DT_CROWDAGENT_TARGET_REQUESTING,
        DT_CROWDAGENT_TARGET_WAITING_FOR_QUEUE,
        DT_CROWDAGENT_TARGET_WAITING_FOR_PATH,
        DT_CROWDAGENT_TARGET_VELOCITY,
    }

    #[repr(u32)]
    enum UpdateFlags {
        DT_CROWD_ANTICIPATE_TURNS = 1,
        DT_CROWD_OBSTACLE_AVOIDANCE = 2,
        DT_CROWD_SEPARATION = 4,
        DT_CROWD_OPTIMIZE_VIS = 8,
        DT_CROWD_OPTIMIZE_TOPO = 16,
    }

    unsafe extern "C++" {
        include!("recastnavigation-rs/src/detour_crowd/crowd-ffi.h");

        type dtNavMesh = crate::detour::mesh::ffi::dtNavMesh;
        type dtQueryFilter = crate::detour::query::DtQueryFilter;
        type dtPathCorridor = crate::detour_crowd::path_corridor::ffi::dtPathCorridor;
        type dtLocalBoundary = crate::detour_crowd::local_boundary::ffi::dtLocalBoundary;
        type dtObstacleAvoidanceParams = crate::detour_crowd::obstacle_avoidance::DtObstacleAvoidanceParams;

        type CrowdAgentState;
        type MoveRequestState;
        type UpdateFlags;

        type dtPolyRef = crate::detour::mesh::DtPolyRef;
        type dtCrowdNeighbour = crate::detour_crowd::crowd::DtCrowdNeighbour;
        type dtCrowdAgentParams = crate::detour_crowd::crowd::DtCrowdAgentParams;

        type dtCrowdAgent = crate::detour_crowd::crowd::CxxDtCrowdAgent;
        fn dtca_getActive(agent: &dtCrowdAgent) -> bool;
        fn dtca_setActive(agent: Pin<&mut dtCrowdAgent>, active: bool);
        fn dtca_getState(agent: &dtCrowdAgent) -> u8;
        fn dtca_setState(agent: Pin<&mut dtCrowdAgent>, state: u8);
        fn dtca_getPartial(agent: &dtCrowdAgent) -> bool;
        fn dtca_setPartial(agent: Pin<&mut dtCrowdAgent>, partial: bool);
        fn dtca_getCorridor(agent: &dtCrowdAgent) -> *const dtPathCorridor;
        fn dtca_getCorridor_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut dtPathCorridor;
        fn dtca_getBoundary(agent: &dtCrowdAgent) -> *const dtLocalBoundary;
        fn dtca_getBoundary_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut dtLocalBoundary;
        fn dtca_getTopologyOptTime(agent: &dtCrowdAgent) -> f32;
        fn dtca_setTopologyOptTime(agent: Pin<&mut dtCrowdAgent>, topologyOptTime: f32);
        fn dtca_getNeis(agent: &dtCrowdAgent) -> *const dtCrowdNeighbour;
        fn dtca_getNeis_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut dtCrowdNeighbour;
        fn dtca_getNneis(agent: &dtCrowdAgent) -> i32;
        fn dtca_getDesiredSpeed(agent: &dtCrowdAgent) -> f32;
        fn dtca_setDesiredSpeed(agent: Pin<&mut dtCrowdAgent>, desiredSpeed: f32);
        fn dtca_getNpos(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setNpos(agent: Pin<&mut dtCrowdAgent>, npos: *const f32);
        fn dtca_getDisp(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setDisp(agent: Pin<&mut dtCrowdAgent>, disp: *const f32);
        fn dtca_getDvel(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setDvel(agent: Pin<&mut dtCrowdAgent>, dvel: *const f32);
        fn dtca_getNvel(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setNvel(agent: Pin<&mut dtCrowdAgent>, nvel: *const f32);
        fn dtca_getVel(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setVel(agent: Pin<&mut dtCrowdAgent>, vel: *const f32);
        fn dtca_getParams(agent: &dtCrowdAgent) -> *const dtCrowdAgentParams;
        fn dtca_getParams_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut dtCrowdAgentParams;
        fn dtca_getCornerVerts(agent: &dtCrowdAgent) -> *const f32;
        fn dtca_getCornerVerts_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut f32;
        fn dtca_getCornerFlags(agent: &dtCrowdAgent) -> *const u8;
        fn dtca_getCornerFlags_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut u8;
        fn dtca_getCornerPolys(agent: &dtCrowdAgent) -> *const dtPolyRef;
        fn dtca_getCornerPolys_mut(agent: Pin<&mut dtCrowdAgent>) -> *mut dtPolyRef;
        fn dtca_getNcorners(agent: &dtCrowdAgent) -> i32;
        fn dtca_getTargetState(agent: &dtCrowdAgent) -> u8;
        fn dtca_setTargetState(agent: Pin<&mut dtCrowdAgent>, targetState: u8);
        fn dtca_getTargetRef(agent: &dtCrowdAgent) -> dtPolyRef;
        fn dtca_setTargetRef(agent: Pin<&mut dtCrowdAgent>, targetRef: dtPolyRef);
        fn dtca_getTargetPos(agent: &dtCrowdAgent) -> *const f32;
        unsafe fn dtca_setTargetPos(agent: Pin<&mut dtCrowdAgent>, targetPos: *const f32);
        fn dtca_getTargetPathqRef(agent: &dtCrowdAgent) -> u32;
        fn dtca_setTargetPathqRef(agent: Pin<&mut dtCrowdAgent>, targetPathqRef: u32);
        fn dtca_getTargetReplan(agent: &dtCrowdAgent) -> bool;
        fn dtca_setTargetReplan(agent: Pin<&mut dtCrowdAgent>, targetReplan: bool);
        fn dtca_getTargetReplanTime(agent: &dtCrowdAgent) -> f32;
        fn dtca_setTargetReplanTime(agent: Pin<&mut dtCrowdAgent>, targetReplanTime: f32);

        type dtCrowdAgentAnimation = crate::detour_crowd::crowd::DtCrowdAgentAnimation;
        type dtCrowdAgentDebugInfo = crate::detour_crowd::crowd::DtCrowdAgentDebugInfo;

        type dtCrowd;
        fn dtAllocCrowd() -> *mut dtCrowd;
        unsafe fn dtFreeCrowd(ptr: *mut dtCrowd);
        unsafe fn init(self: Pin<&mut dtCrowd>, maxAgents: i32, maxAgentRadius: f32, nav: *mut dtNavMesh) -> bool;
        unsafe fn setObstacleAvoidanceParams(
            self: Pin<&mut dtCrowd>,
            idx: i32,
            params: *const dtObstacleAvoidanceParams,
        );
        fn getObstacleAvoidanceParams(self: &dtCrowd, idx: i32) -> *const dtObstacleAvoidanceParams;
        fn getAgent(self: Pin<&mut dtCrowd>, idx: i32) -> *const dtCrowdAgent;
        fn getEditableAgent(self: Pin<&mut dtCrowd>, idx: i32) -> *mut dtCrowdAgent;
        fn getAgentCount(self: &dtCrowd) -> i32;
        unsafe fn addAgent(self: Pin<&mut dtCrowd>, pos: *const f32, params: *const dtCrowdAgentParams) -> i32;
        unsafe fn updateAgentParameters(self: Pin<&mut dtCrowd>, idx: i32, params: *const dtCrowdAgentParams);
        fn removeAgent(self: Pin<&mut dtCrowd>, idx: i32);
        unsafe fn requestMoveTarget(self: Pin<&mut dtCrowd>, idx: i32, re: dtPolyRef, pos: *const f32) -> bool;
        unsafe fn requestMoveVelocity(self: Pin<&mut dtCrowd>, idx: i32, vel: *const f32) -> bool;
        fn resetMoveTarget(self: Pin<&mut dtCrowd>, idx: i32) -> bool;
        unsafe fn getActiveAgents(self: Pin<&mut dtCrowd>, agents: *mut *mut dtCrowdAgent, maxAgents: i32) -> i32;
        unsafe fn update(self: Pin<&mut dtCrowd>, dt: f32, debug: *mut dtCrowdAgentDebugInfo);
        fn getFilter(self: &dtCrowd, i: i32) -> *const dtQueryFilter;
        fn getEditableFilter(self: Pin<&mut dtCrowd>, i: i32) -> *mut dtQueryFilter;
        fn getQueryHalfExtents(self: &dtCrowd) -> *const f32;
        fn getQueryExtents(self: &dtCrowd) -> *const f32;
        fn getVelocitySampleCount(self: &dtCrowd) -> i32;
        // fn getGrid(self: &dtCrowd) -> *const dtProximityGrid;
        // fn getPathQueue(self: &dtCrowd) -> *const dtPathQueue;
        // fn getNavMeshQuery(self: &dtCrowd) -> *const dtNavMeshQuery;
    }
}

pub type CrowdAgentState = ffi::CrowdAgentState;
pub type MoveRequestState = ffi::MoveRequestState;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DtCrowdNeighbour {
    pub idx: i32,
    pub dist: f32,
}

const_assert_eq!(std::mem::size_of::<DtCrowdNeighbour>(), 8);

unsafe impl ExternType for DtCrowdNeighbour {
    type Id = type_id!("dtCrowdNeighbour");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DtCrowdAgentParams {
    pub radius: f32,
    pub height: f32,
    pub max_acceleration: f32,
    pub max_speed: f32,
    pub collision_query_range: f32,
    pub path_optimization_range: f32,
    pub separation_weight: f32,
    pub update_flags: u8,
    pub obstacle_avoidance_type: u8,
    pub query_filter_type: u8,
    pub user_data: *mut std::os::raw::c_void,
}

unsafe impl ExternType for DtCrowdAgentParams {
    type Id = type_id!("dtCrowdAgentParams");
    type Kind = cxx::kind::Trivial;
}

impl Default for DtCrowdAgentParams {
    fn default() -> Self {
        DtCrowdAgentParams {
            radius: 0.6,
            height: 2.0,
            max_acceleration: 8.0,
            max_speed: 3.5,
            collision_query_range: 12.0,
            path_optimization_range: 30.0,
            separation_weight: 2.0,
            update_flags: 0,
            obstacle_avoidance_type: 3,
            query_filter_type: 0,
            user_data: std::ptr::null_mut(),
        }
    }
}

#[repr(C, align(8))]
pub struct CxxDtCrowdAgent([u8; 624]);

unsafe impl ExternType for CxxDtCrowdAgent {
    type Id = type_id!("dtCrowdAgent");
    type Kind = cxx::kind::Trivial;
}

pub struct DtCrowdAgent(CxxDtCrowdAgent);

impl Debug for DtCrowdAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return f
            .debug_tuple("DtCrowdAgent")
            .field(&(self as *const DtCrowdAgent))
            .finish();
    }
}

impl DtCrowdAgent {
    #[inline]
    fn inner(&self) -> &ffi::dtCrowdAgent {
        &self.0
    }

    #[inline]
    fn inner_mut(&mut self) -> Pin<&mut ffi::dtCrowdAgent> {
        unsafe { Pin::new_unchecked(&mut self.0) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::dtCrowdAgent {
        &self.0
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::dtCrowdAgent {
        &mut self.0
    }

    #[inline]
    pub fn active(&self) -> bool {
        return ffi::dtca_getActive(self.inner());
    }

    #[inline]
    pub fn set_active(&mut self, active: bool) {
        ffi::dtca_setActive(self.inner_mut(), active);
    }

    #[inline] // TODO: CrowdAgentState
    pub fn state(&self) -> u8 {
        return ffi::dtca_getState(self.inner());
    }

    #[inline]
    pub fn set_state(&mut self, state: u8) {
        ffi::dtca_setState(self.inner_mut(), state);
    }

    #[inline]
    pub fn partial(&self) -> bool {
        return ffi::dtca_getPartial(self.inner());
    }

    #[inline]
    pub fn set_partial(&mut self, partial: bool) {
        ffi::dtca_setPartial(self.inner_mut(), partial);
    }

    #[inline]
    pub fn corridor(&self) -> &DtPathCorridor {
        return unsafe { &*(ffi::dtca_getCorridor(self.inner()) as *const DtPathCorridor) };
    }

    #[inline]
    pub fn corridor_mut(&mut self) -> &mut DtPathCorridor {
        return unsafe { &mut *(ffi::dtca_getCorridor_mut(self.inner_mut()) as *mut DtPathCorridor) };
    }

    #[inline]
    pub fn boundary(&self) -> &DtLocalBoundary {
        return unsafe { &*(ffi::dtca_getBoundary(self.inner()) as *const DtLocalBoundary) };
    }

    #[inline]
    pub fn boundary_mut(&mut self) -> &mut DtLocalBoundary {
        return unsafe { &mut *(ffi::dtca_getBoundary_mut(self.inner_mut()) as *mut DtLocalBoundary) };
    }

    #[inline]
    pub fn topology_opt_time(&self) -> f32 {
        return ffi::dtca_getTopologyOptTime(self.inner());
    }

    #[inline]
    pub fn set_topology_opt_time(&mut self, topology_opt_time: f32) {
        ffi::dtca_setTopologyOptTime(self.inner_mut(), topology_opt_time);
    }

    #[inline]
    pub fn neis(&self) -> &[DtCrowdNeighbour] {
        return unsafe {
            std::slice::from_raw_parts(
                ffi::dtca_getNeis(self.inner()),
                ffi::dtca_getNneis(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn neis_mut(&mut self) -> &mut [DtCrowdNeighbour] {
        return unsafe {
            std::slice::from_raw_parts_mut(
                ffi::dtca_getNeis_mut(self.inner_mut()),
                ffi::dtca_getNneis(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn desired_speed(&self) -> f32 {
        return ffi::dtca_getDesiredSpeed(self.inner());
    }

    #[inline]
    pub fn set_desired_speed(&mut self, desired_speed: f32) {
        ffi::dtca_setDesiredSpeed(self.inner_mut(), desired_speed);
    }

    #[inline]
    pub fn npos(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getNpos(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_npos(&mut self, npos: &[f32; 3]) {
        unsafe { ffi::dtca_setNpos(self.inner_mut(), npos.as_ptr()) };
    }

    #[inline]
    pub fn disp(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getDisp(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_disp(&mut self, disp: &[f32; 3]) {
        unsafe { ffi::dtca_setDisp(self.inner_mut(), disp.as_ptr()) };
    }

    #[inline]
    pub fn dvel(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getDvel(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_dvel(&mut self, dvel: &[f32; 3]) {
        unsafe { ffi::dtca_setDvel(self.inner_mut(), dvel.as_ptr()) };
    }

    #[inline]
    pub fn nvel(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getNvel(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_nvel(&mut self, nvel: &[f32; 3]) {
        unsafe { ffi::dtca_setNvel(self.inner_mut(), nvel.as_ptr()) };
    }

    #[inline]
    pub fn vel(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getVel(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_vel(&mut self, vel: &[f32; 3]) {
        unsafe { ffi::dtca_setVel(self.inner_mut(), vel.as_ptr()) };
    }

    #[inline]
    pub fn params(&self) -> &DtCrowdAgentParams {
        return unsafe { &*ffi::dtca_getParams(self.inner()) };
    }

    #[inline]
    pub fn params_mut(&mut self) -> &mut DtCrowdAgentParams {
        return unsafe { &mut *ffi::dtca_getParams_mut(self.inner_mut()) };
    }

    #[inline]
    pub fn corner_verts(&self) -> &[[f32; 3]] {
        return unsafe {
            std::slice::from_raw_parts(
                ffi::dtca_getCornerVerts(self.inner()) as *const _,
                ffi::dtca_getNcorners(self.inner()) as usize * 3,
            )
        };
    }

    #[inline]
    pub fn corner_verts_mut(&mut self) -> &mut [[f32; 3]] {
        return unsafe {
            std::slice::from_raw_parts_mut(
                ffi::dtca_getCornerVerts_mut(self.inner_mut()) as *mut _,
                ffi::dtca_getNcorners(self.inner()) as usize * 3,
            )
        };
    }

    #[inline]
    pub fn corner_flags(&self) -> &[u8] {
        return unsafe {
            std::slice::from_raw_parts(
                ffi::dtca_getCornerFlags(self.inner()),
                ffi::dtca_getNcorners(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn corner_flags_mut(&mut self) -> &mut [u8] {
        return unsafe {
            std::slice::from_raw_parts_mut(
                ffi::dtca_getCornerFlags_mut(self.inner_mut()),
                ffi::dtca_getNcorners(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn corner_polys(&self) -> &[DtPolyRef] {
        return unsafe {
            std::slice::from_raw_parts(
                ffi::dtca_getCornerPolys(self.inner()),
                ffi::dtca_getNcorners(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn corner_polys_mut(&mut self) -> &mut [DtPolyRef] {
        return unsafe {
            std::slice::from_raw_parts_mut(
                ffi::dtca_getCornerPolys_mut(self.inner_mut()),
                ffi::dtca_getNcorners(self.inner()) as usize,
            )
        };
    }

    #[inline]
    pub fn ncorners(&self) -> i32 {
        return ffi::dtca_getNcorners(self.inner());
    }

    #[inline]
    pub fn target_state(&self) -> u8 {
        return ffi::dtca_getTargetState(self.inner());
    }

    #[inline]
    pub fn set_target_state(&mut self, target_state: u8) {
        ffi::dtca_setTargetState(self.inner_mut(), target_state);
    }

    #[inline]
    pub fn target_ref(&self) -> DtPolyRef {
        return ffi::dtca_getTargetRef(self.inner());
    }

    #[inline]
    pub fn set_target_ref(&mut self, target_ref: DtPolyRef) {
        ffi::dtca_setTargetRef(self.inner_mut(), target_ref);
    }

    #[inline]
    pub fn target_pos(&self) -> &[f32; 3] {
        return unsafe { &*(ffi::dtca_getTargetPos(self.inner()) as *const [f32; 3]) };
    }

    #[inline]
    pub fn set_target_pos(&mut self, target_pos: &[f32; 3]) {
        unsafe { ffi::dtca_setTargetPos(self.inner_mut(), target_pos.as_ptr()) };
    }

    #[inline]
    pub fn target_pathq_ref(&self) -> u32 {
        return ffi::dtca_getTargetPathqRef(self.inner());
    }

    #[inline]
    pub fn set_target_pathq_ref(&mut self, target_pathq_ref: u32) {
        ffi::dtca_setTargetPathqRef(self.inner_mut(), target_pathq_ref);
    }

    #[inline]
    pub fn target_replan(&self) -> bool {
        return ffi::dtca_getTargetReplan(self.inner());
    }

    #[inline]
    pub fn set_target_replan(&mut self, target_replan: bool) {
        ffi::dtca_setTargetReplan(self.inner_mut(), target_replan);
    }

    #[inline]
    pub fn target_replan_time(&self) -> f32 {
        return ffi::dtca_getTargetReplanTime(self.inner());
    }

    #[inline]
    pub fn set_target_replan_time(&mut self, target_replan_time: f32) {
        ffi::dtca_setTargetReplanTime(self.inner_mut(), target_replan_time);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DtCrowdAgentAnimation {
    pub active: bool,
    pub init_pos: [f32; 3],
    pub start_pos: [f32; 3],
    pub end_pos: [f32; 3],
    pub poly_ref: DtPolyRef,
    pub t: f32,
    pub tmax: f32,
}

unsafe impl ExternType for DtCrowdAgentAnimation {
    type Id = type_id!("dtCrowdAgentAnimation");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DtCrowdAgentDebugInfo {
    pub idx: i32,
    pub opt_start: [f32; 3],
    pub opt_end: [f32; 3],
    vod: *mut (), // TODO: debug data
}

unsafe impl ExternType for DtCrowdAgentDebugInfo {
    type Id = type_id!("dtCrowdAgentDebugInfo");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug)]
pub struct DtCrowd(*mut ffi::dtCrowd);

impl Drop for DtCrowd {
    fn drop(&mut self) {
        unsafe { ffi::dtFreeCrowd(self.0) };
        self.0 = std::ptr::null_mut();
    }
}

impl Default for DtCrowd {
    fn default() -> Self {
        Self::new()
    }
}

impl DtCrowd {
    #[inline]
    pub fn new() -> DtCrowd {
        DtCrowd(ffi::dtAllocCrowd())
    }

    #[inline]
    fn inner(&self) -> &ffi::dtCrowd {
        unsafe { &*self.0 }
    }

    #[inline]
    fn inner_mut(&mut self) -> Pin<&mut ffi::dtCrowd> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::dtCrowd {
        self.0
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::dtCrowd {
        self.0
    }

    #[inline]
    pub fn init(&mut self, max_agents: i32, max_agent_radius: f32, nav: &mut DtNavMesh) -> bool {
        return unsafe { self.inner_mut().init(max_agents, max_agent_radius, nav.as_mut_ptr()) };
    }

    #[inline]
    pub fn set_obstacle_avoidance_params(&mut self, idx: i32, params: &DtObstacleAvoidanceParams) {
        unsafe { self.inner_mut().setObstacleAvoidanceParams(idx, params) };
    }

    #[inline]
    pub fn get_obstacle_avoidance_params(&self, idx: i32) -> &DtObstacleAvoidanceParams {
        return unsafe { &*self.inner().getObstacleAvoidanceParams(idx) };
    }

    // pub fn get_agent(&self, idx: i32) -> &DtCrowdAgent {
    //     return unsafe { mem::transmute(self.inner().get) };
    // }

    #[inline]
    pub fn get_agent_mut(&mut self, idx: i32) -> &mut DtCrowdAgent {
        return unsafe { &mut *(self.inner_mut().getEditableAgent(idx) as *mut DtCrowdAgent) };
    }

    #[inline]
    pub fn get_agent_count(&self) -> i32 {
        return self.inner().getAgentCount();
    }

    #[inline]
    pub fn add_agent(&mut self, pos: &[f32; 3], params: &DtCrowdAgentParams) -> i32 {
        return unsafe { self.inner_mut().addAgent(pos as *const _, params) };
    }

    #[inline]
    pub fn update_agent_parameters(&mut self, idx: i32, params: &DtCrowdAgentParams) {
        unsafe { self.inner_mut().updateAgentParameters(idx, params) };
    }

    #[inline]
    pub fn remove_agent(&mut self, idx: i32) {
        self.inner_mut().removeAgent(idx);
    }

    #[inline]
    pub fn request_move_target(&mut self, idx: i32, re: DtPolyRef, pos: &[f32; 3]) -> bool {
        return unsafe { self.inner_mut().requestMoveTarget(idx, re, pos as *const _) };
    }

    #[inline]
    pub fn request_move_velocity(&mut self, idx: i32, vel: &[f32; 3]) -> bool {
        return unsafe { self.inner_mut().requestMoveVelocity(idx, vel as *const _) };
    }

    #[inline]
    pub fn reset_move_target(&mut self, idx: i32) -> bool {
        return self.inner_mut().resetMoveTarget(idx);
    }

    #[inline]
    pub fn get_active_agents(&mut self, agents: &mut [&mut DtCrowdAgent]) -> i32 {
        return unsafe {
            self.inner_mut()
                .getActiveAgents(agents.as_mut_ptr() as *mut _, agents.len() as i32)
        };
    }

    #[inline]
    pub fn update(&mut self, dt: f32, debug: &mut DtCrowdAgentDebugInfo) {
        unsafe { self.inner_mut().update(dt, debug) };
    }

    #[inline]
    pub fn filter(&self, i: i32) -> &DtQueryFilter {
        return unsafe { &*self.inner().getFilter(i) };
    }

    #[inline]
    pub fn filter_mut(&mut self, i: i32) -> &mut DtQueryFilter {
        return unsafe { &mut *self.inner_mut().getEditableFilter(i) };
    }

    #[inline]
    pub fn query_half_extents(&self) -> &[f32; 3] {
        return unsafe { &*(self.inner().getQueryHalfExtents() as *const [f32; 3]) };
    }

    #[inline]
    pub fn query_extents(&self) -> &[f32; 3] {
        return unsafe { &*(self.inner().getQueryExtents() as *const [f32; 3]) };
    }

    #[inline]
    pub fn velocity_sample_count(&self) -> i32 {
        return self.inner().getVelocitySampleCount();
    }
}
