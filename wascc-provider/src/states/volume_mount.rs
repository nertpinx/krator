use kubelet::state::{PodChangeRx, State, Transition};
use kubelet::volume::Ref;
use kubelet::{
    pod::{Phase, Pod},
    state,
};

use crate::{make_status, PodState};

use super::error::Error;
use super::starting::Starting;

state!(
    /// Kubelet is pulling container images.
    VolumeMount,
    PodState,
    Starting,
    Error,
    {
        pod_state.run_context.volumes =
            Ref::volumes_from_pod(&pod_state.volume_path, &pod, &pod_state.client)
                .await
                .unwrap();
        Ok(Transition::Advance(Starting))
    },
    { make_status(Phase::Pending, "VolumeMount") }
);
