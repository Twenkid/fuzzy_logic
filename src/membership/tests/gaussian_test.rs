use super::gaussian::GaussianMF;
#[test]
fn it_works() {
    let r = GaussianMF::new(20, 4, 50);
    r.call(5);
    r.get_name();
    r.get_params();
}
