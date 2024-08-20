struct Data {
    name: String,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping Data: {}", self.name);
    }
}

fn main() {
    let mut data1 = Box::new(Data { name: "A".to_string() });
    let mut data2 = Box::new(Data { name: "B".to_string() });

    println!("Data1: {}", data1.name);
    println!("Data2: {}", data2.name);

    {
        let data3 = Box::new(Data { name: "C".to_string() });
        println!("Data3: {}", data3.name);

        data1 = data2;
        data2 = data3;
        println!("Data1: {}", data1.name);
        println!("Data2: {}", data2.name);
    }

    println!("Data1: {}", data1.name);
    println!("Data2: {}", data2.name);
}
