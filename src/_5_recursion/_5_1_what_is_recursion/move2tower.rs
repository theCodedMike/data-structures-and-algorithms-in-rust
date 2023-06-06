/// p: pole 杆
pub fn move2tower(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height == 0 {
        return;
    }
    move2tower(height - 1, src_p, mid_p, des_p);
    println!("moving disk from {src_p} to {des_p}");
    move2tower(height - 1, mid_p, des_p, src_p);
}
