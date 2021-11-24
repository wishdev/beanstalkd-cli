use beanstalkd::Beanstalkd;

pub fn pop(beanstalkd: &mut Beanstalkd, tubes: Vec<&str>, output_id: bool) {
    if !(tubes.len() == 1 && tubes[0] == "default") {
        let _ = beanstalkd.ignore("default");
        for tube in tubes {
            let _ = beanstalkd.watch(tube);
        }
    }
    let (id, message) = beanstalkd.reserve().unwrap();
    if output_id {
        println!("{} {}", id, message);
    } else {
        println!("{}", message);
    }
    let _ = beanstalkd.delete(id);
}
