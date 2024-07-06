mod pocket_state;

fn main() {
    let pocket = pocket_state::Pocket::default();
    dbg!(&pocket);
}
