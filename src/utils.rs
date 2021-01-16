extern crate js_sys;

/*
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
*/

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn get_random_int(min: f64, max: f64) -> i64 {
    return (min + max * js_sys::Math::random()) as i64;
}

/* spawners

pub fn spawn_glider_at(&mut self, cx: usize, cy: usize, dir: usize) {
    log!("spawn dir{} glider at ({},{})", dir,cx,cy);
    let cxm = (cx-1+self.width ) % self.width;  // wrap edges
    let cxp = (cx+1)             % self.width;
    let cym = (cy-1+self.height) % self.height;
    let cyp = (cy+1)             % self.height;
    let cells: [(usize,usize); 5];
    match dir {
        0 => { // southeasterly
            cells = [(cym,cxm), (cym,cxp), (cy,cx), (cy,cxp), (cyp,cx)];
        }
        1 => { // northwesterly
            cells = [(cyp,cxp), (cyp,cxm), (cy,cx), (cy,cxm), (cym,cx)];
        }
        2 => { // northeasterly
            cells = [(cyp,cxm), (cym,cxm), (cy,cx), (cym,cx), (cy,cxp)];
        }
        _ => { // southwesterly
            cells = [(cym,cxp), (cyp,cxp), (cy,cx), (cyp,cx), (cy,cxm)];
        }
    }
    self.set_cells(&cells);
}

*/
