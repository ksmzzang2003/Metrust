use macroquad::prelude::*;

struct Station {
    circle: Circle,
}
impl Station {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self {
            circle: Circle::new(x, y, r),
        }
    }
    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, BLUE);
    }
}
impl PartialEq for Station {
    fn eq (&self,other : &Self) -> bool {
        return (self.circle.x == other.circle.x)&&(self.circle.y == other.circle.y)&&(self.circle.r == other.circle.r);
    }
}

struct Train {
    circle: Circle,
    next : Option<Station>,
    people : Vec<Client>,
}

impl Train {
    pub fn new(x: f32, y: f32,next : Option<Station>) -> Self {
        Self {
            circle: Circle::new(x, y, 10.0f32),
            next : next, 
            people : Vec::new(),
        }
    }
    pub fn update_train(&mut self, dt: f32) {
        let mut vel : Vec2 = Vec2::new(0f32,0f32);
        if self.next.is_some() {
            let nxt = self.next.as_ref().unwrap();
            let mut dir = const_vec2!([nxt.circle.x,nxt.circle.y]) - const_vec2!([self.circle.x,self.circle.y]);
            vel = dir.normalize();
        }
        self.circle.x += dt * vel.x * 30.0; 
        self.circle.y += dt * vel.y * 30.0; 
    }
    pub fn draw_train(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, RED);
    }
}

struct Line {
    stops: Vec<Station>,
    trains: Vec<Train>,
    next_idx : Vec<usize>,
}

impl Line {
    pub fn new() -> Self {
        Self {
            stops: Vec::new(),
            trains: Vec::new(),
            next_idx : Vec::new(),
        }
    }
    pub fn draw_line(&self) {
        let mut i: usize = 0;
        while (i + 1) < self.stops.len() {
            draw_line(
                self.stops[i].circle.x,
                self.stops[i].circle.y,
                self.stops[i + 1].circle.x,
                self.stops[i + 1].circle.y,
                5f32,
                BLUE,
            );
            i = i + 1;
        }
    }
    pub fn draw_train(&self) {
        for car in self.trains.iter() {
            car.draw_train();
        }
    }
    
    pub fn release_train(&mut self) {        
        //println!("trains len {}",self.trains.len());
        self.trains.push(Train::new(self.stops[0].circle.x, self.stops[0].circle.y,Some(Station::new(self.stops[1].circle.x, self.stops[1].circle.y, self.stops[1].circle.r))));
        self.next_idx.push(1); 
        //println!("new trains len {}",self.trains.len());
        //println!("tf : {}",self.trains[self.trains.len()- 1 as usize].next.is_some());
        //self.trains.push(Train::new(self.trains[0].circle.x, self.trains[0].circle.y, self.trains[1]));
    }

    pub fn circularize(&mut self) {
        //println!("Check stops {}",self.stops.len());
        if (self.stops[0].circle.x == self.stops[self.stops.len()-1].circle.x) && (self.stops[0].circle.y == self.stops[self.stops.len()-1].circle.y) {
            return ; 
        }
        else {
            println!("Is it alive? {}",self.stops.len());
            let mut i = self.stops.len() -2 ;
            while i>=0 {
                self.stops.push(Station::new(self.stops[i].circle.x,self.stops[i].circle.y,self.stops[i].circle.r));
                if i==0 {
                    break; 
                } 
                i=i-1; 
            }
        }
    }

    pub fn update(&mut self, dt : f32){
        //println!("# of train : {}", self.trains.len());
        let mut i = 0; 
        while i < self.trains.len() {
            self.trains[i].update_train(dt); 
            if self.trains[i].next.is_some() {
                let nxt = self.trains[i].next.as_ref().unwrap();
                if const_vec2!([nxt.circle.x,nxt.circle.y]).distance(const_vec2!([self.trains[i].circle.x,self.trains[i].circle.y])) < 1.0 {
                    let mut idx = self.next_idx[self.next_idx.len() -1 as usize]+1; 
                    if idx == self.stops.len() { 
                        idx  = 1 ;
                        self.next_idx.clear();
                    }
                    self.next_idx.push(idx);
                    self.trains[i].next = Some(Station::new(self.stops[idx].circle.x, self.stops[idx].circle.y, self.stops[idx].circle.r));
                }
            }
            i = i+1;
        }
    }
}


struct Client {
    source : Station, 
    sink : Station,  
}
impl Client {
    pub fn new(S : Station, T : Station) -> Self {
        Self {
            source : S, 
            sink : T, 
        }
    }
}
#[macroquad::main("Metrust")]
async fn main() {
    let mut Stations: Vec<Station> = vec![];
    
    let mut Lines: Vec<Line> = vec![Line::new()];
    loop {
        clear_background(WHITE);
        // Line Linking
        if is_key_down(KeyCode::Space)==false && is_mouse_button_pressed(MouseButton::Left) {
            Stations.push(Station::new(
                mouse_position().0,
                mouse_position().1,
                20f32,
            ),)
        }
        if is_key_down(KeyCode::Space) {
            if is_mouse_button_pressed(MouseButton::Left) {
                //println!("Is it released?");
                for stop in Stations.iter() {
                    if const_vec2!([stop.circle.x, stop.circle.y])
                        .distance(const_vec2!([mouse_position().0, mouse_position().1]))
                        < stop.circle.r * 0.7
                    {
                        if Lines[0].stops.len() ==0 {
                            Lines[0].stops.push(Station::new(
                                stop.circle.x,
                                stop.circle.y,
                                stop.circle.r,
                            ));
                        }
                        else if (Station::new(
                            stop.circle.x,
                            stop.circle.y,
                            stop.circle.r,
                        ) != Lines[0].stops[Lines[0].stops.len() -1 as usize]){
                            Lines[0].stops.push(Station::new(
                                stop.circle.x,
                                stop.circle.y,
                                stop.circle.r,
                            ));
                        }
                    }
                }
                // for stop in Lines[0].stops.iter() {
                //     println!("Check : {},{} radius : {} ", stop.circle.x ,stop.circle.y,stop.circle.r);
                // }
                // println!("Checking End");
            }
            //println!("Length : {}",Lines[0].stops.len());
            if Lines[0].stops.len()>0 {
                draw_line(Lines[0].stops[Lines[0].stops.len() - 1 as usize].circle.x, Lines[0].stops[Lines[0].stops.len() - 1 as usize].circle.y, mouse_position().0, mouse_position().1, 5.0f32, BLUE);
            }
        }

        for line in Lines.iter_mut() {
            if is_mouse_button_pressed(MouseButton::Right) {
                println!("Right Click released!");
                line.circularize();
                line.release_train(); 
                println!("next {} ",line.trains[0].next.is_some());
            }
            line.draw_line();
            line.draw_train();
            //println!("Update Reached?");
            line.update(get_frame_time()); 
        }

        let mut i = 0; 
        while i < Stations.len() {
            Stations[i].draw(); 
            draw_text(i.to_string().as_str(), Stations[i].circle.x, Stations[i].circle.y, 30f32, BLACK);
            i = i+1; 
        }

        next_frame().await;
    }
}
