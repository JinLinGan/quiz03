fn main() {
    {
        let l1 = [1, 2, 3].as_slice();
        let r1 = count_sum(l1);
        println!("sum {:?} get {:?}", l1, r1);
    }

    {
        let l1 = [u32::MAX].as_slice();
        let r1 = count_sum(l1);
        println!("sum {:?} get {:?}", l1, r1);
    }

    {
        let l1 = [u32::MAX,1].as_slice();
        let r1 = count_sum(l1);
        println!("sum {:?} get {:?}", l1, r1);
    }
}

fn count_sum(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in list.iter() {
        if u32::MAX - i < sum {
            return None;
        }
        sum += i;
    }
    Some(sum)
}
