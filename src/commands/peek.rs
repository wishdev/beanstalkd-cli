use beanstalkd::Beanstalkd;

pub fn peek(beanstalkd: &mut Beanstalkd, type_or_id: String, tube: &str, output_id: bool, output_id_only: bool) {
    if tube != "default" {
        let _ = beanstalkd.tube(tube);
    }

    let (id, message) = beanstalkd.peek(&type_or_id).unwrap();
    if output_id_only {
        println!("{}", id);
    } else if output_id {
        println!("{} {}", id, message);
    } else {
        println!("{}", message);
    }
}
