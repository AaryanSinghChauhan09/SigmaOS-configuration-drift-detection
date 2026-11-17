//! Integration tests for SigmaOS-configuration-drift-detection.

use sigmaos_configuration_drift_detection_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
