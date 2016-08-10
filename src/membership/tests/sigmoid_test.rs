use super::sigmoid::SigmoidMF;
#[test]
fn it_works() {
    let s = SigmoidMF::new(2, 3, 4);
    s.call(5);
    s.get_name();
    s.get_params();
}
