#![allow(dead_code)]
pub struct Stigmergy { grid: [[f64; 64]; 64], kinds: [[u8; 64]; 64], deposited: [[u32; 64]; 64], decay_rate: f64, diffusion: f64 }
impl Stigmergy {
    pub fn new(decay: f64) -> Self { Self { grid: [[0.0; 64]; 64], kinds: [[0; 64]; 64], deposited: [[0; 64]; 64], decay_rate: decay, diffusion: 0.01 } }
    pub fn deposit(&mut self, x: i16, y: i16, strength: f64, kind: u8, _agent: u32) {
        if x < 0 || x >= 64 || y < 0 || y >= 64 { return; }
        let (ux, uy) = (x as usize, y as usize);
        self.grid[uy][ux] = (self.grid[uy][ux] + strength).min(100.0);
        self.kinds[uy][ux] = kind;
        self.deposited[uy][ux] += 1;
    }
    pub fn read(&self, x: i16, y: i16) -> (f64, u8) {
        if x < 0 || x >= 64 || y < 0 || y >= 64 { return (0.0, 0); }
        (self.grid[y as usize][x as usize], self.kinds[y as usize][x as usize])
    }
    pub fn read_area(&self, x: i16, y: i16, radius: i16) -> (f64, u8) {
        let mut total = 0.0; let mut count = 0u32; let mut kind_counts = [0u32; 256];
        for dy in -radius..=radius { for dx in -radius..=radius { if dx*dx + dy*dy <= radius*radius {
            let (s, k) = self.read(x + dx, y + dy); total += s; if s > 0.0 { count += 1; kind_counts[k as usize] += 1; }
        }}}
        let avg = if count > 0 { total / count as f64 } else { 0.0 };
        let dominant = kind_counts.iter().enumerate().max_by_key(|(_, &c)| c).map(|(k, _)| k as u8).unwrap_or(0);
        (avg, dominant)
    }
    pub fn decay(&mut self) { for row in &mut self.grid { for v in row { *v = (*v * (1.0 - self.decay_rate)).max(0.0); } } }
    pub fn gradient(&self, x: i16, y: i16, kind: u8) -> (i16, i16) {
        let mut bx = 0i16; let mut by = 0i16; let mut best = self.read(x, y).0;
        for (dx, dy) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            let nx = x + dx; let ny = y + dy;
            let (s, k) = self.read(nx, ny);
            if k == kind && s > best { best = s; bx = *dx; by = *dy; }
        } (bx, by)
    }
    pub fn clear(&mut self, x: i16, y: i16) { if x >= 0 && x < 64 && y >= 0 && y < 64 { self.grid[y as usize][x as usize] = 0.0; } }
    pub fn set_decay(&mut self, rate: f64) { self.decay_rate = rate; }
    pub fn strongest(&self, kind: u8) -> Option<(i16, i16)> {
        let mut best = 0.0; let mut pos = None;
        for y in 0..64u16 { for x in 0..64u16 { if self.kinds[y as usize][x as usize] == kind && self.grid[y as usize][x as usize] > best {
            best = self.grid[y as usize][x as usize]; pos = Some((x as i16, y as i16));
        }}} pos
    }
    pub fn total(&self, kind: u8) -> f64 { let mut t = 0.0; for y in 0..64 { for x in 0..64 { if self.kinds[y][x] == kind { t += self.grid[y][x]; } } } t }
    pub fn evaporate(&mut self, amount: f64) { for row in &mut self.grid { for v in row { *v = (*v - amount).max(0.0); } } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let s = Stigmergy::new(0.01); assert!((s.read(32, 32).0).abs() < 1e-6); }
    #[test] fn test_deposit_read() { let mut s = Stigmergy::new(0.01); s.deposit(10, 10, 5.0, 1, 0); assert!((s.read(10, 10).0 - 5.0).abs() < 1e-6); }
    #[test] fn test_out_of_bounds() { let mut s = Stigmergy::new(0.01); s.deposit(-1, -1, 5.0, 1, 0); assert_eq!(s.read(-1, -1).0, 0.0); }
    #[test] fn test_decay() { let mut s = Stigmergy::new(0.1); s.deposit(5, 5, 10.0, 1, 0); s.decay(); assert!(s.read(5, 5).0 < 10.0); }
    #[test] fn test_gradient() { let mut s = Stigmergy::new(0.01); s.deposit(10, 10, 1.0, 1, 0); s.deposit(11, 10, 5.0, 1, 0); let (gx, gy) = s.gradient(10, 10, 1); assert_eq!(gx, 1); }
    #[test] fn test_clear() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 10.0, 1, 0); s.clear(5, 5); assert!((s.read(5, 5).0).abs() < 1e-6); }
    #[test] fn test_strongest() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 3.0, 1, 0); s.deposit(10, 10, 7.0, 1, 0); assert_eq!(s.strongest(1), Some((10, 10))); }
    #[test] fn test_total() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 3.0, 1, 0); s.deposit(10, 10, 7.0, 1, 0); assert!((s.total(1) - 10.0).abs() < 1e-6); }
    #[test] fn test_evaporate() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 10.0, 1, 0); s.evaporate(5.0); assert!((s.read(5, 5).0 - 5.0).abs() < 1e-6); }
    #[test] fn test_read_area() { let mut s = Stigmergy::new(0.01); s.deposit(10, 10, 10.0, 1, 0); let (avg, kind) = s.read_area(10, 10, 2); assert!(avg > 0.0); assert_eq!(kind, 1); }
    #[test] fn test_kind_isolation() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 10.0, 1, 0); s.deposit(5, 5, 10.0, 2, 0); assert_eq!(s.total(1), 10.0); }
    #[test] fn test_saturation() { let mut s = Stigmergy::new(0.01); s.deposit(5, 5, 200.0, 1, 0); assert!(s.read(5, 5).0 <= 100.0); }
}