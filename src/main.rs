
use std::f32::consts::PI;


use ortho_rotation::ortho_rotate_point;


fn main() {
    
    let mut square : Vec<Vec<u32>> = Vec::new();
    let mut newsquare: Vec<Vec<u32>> = Vec::new();
    let mut counter = 100;
    
    //filling the new square with "888" to represent values that are not mapped to after rotating square
    for x in 0..9{
        let mut newsquarevec = Vec::new();
        for y in 0..9{
            newsquarevec.push(888);
        }
        newsquare.push(newsquarevec);
    }
    
    //filling square with 100 - 180 and newsquare by rotating each point of the old square
    for x in 0..9{
        
        let mut squarevec = Vec::new();
        
        for y in 0..9{
            
            squarevec.push(counter);
            
            let posx = x - 4;
            let posy = y - 4;
            
            let amounttorotate = 0.0486;
            let newpos = ortho_rotate_point( (posx as f32, posy as f32) , amounttorotate);
            
            let newposx = newpos.0.round() + 4.0;
            let newposy = newpos.1.round() + 4.0;
            
            let vecx = newposx as usize;
            let vecy = newposy as usize;
            
            newsquare[vecx][vecy] = counter;
            
            counter += 1;
        };
        
        square.push(squarevec);
    };
    
    
    print_2d_vector(square);
    println!("");
    print_2d_vector(newsquare);
    

    //not plotting anything currently
    /*
    use poloto::*;

    let mut data = Vec::new();
    for x in -500..500{

        let float = 0.001 * x as f32;

        data.push( [ float, float ] );
    }

    let mut s = poloto::plot("simple", "x", "y");
    s.scatter("", data.into_iter());

    let file = std::fs::File::create("write_to_file.svg").unwrap();
    s.render(file);
    */
    
    
}


//print a 2d vector of u32s
fn print_2d_vector(vec: Vec<Vec<u32>>){
    
    for x in vec{
        for y in x{
            print!(" {:?} ", y);
        }
        println!("");
    }
}







mod ortho_rotation{

    
    //rotate this point by this amount and return it
    pub fn ortho_rotate_point( point: (f32,f32), torotate: f32) -> (f32,f32){
    
        if point.0 == point.1  && point.0 == 0.0{
            return (0.0,0.0) ;
        }
    
        for numrotation in 0..4{
    
            //rotate the point clockwise x times (negative direction)
            let (rotx,roty) = rotate_point_90_degrees_x_times( point , - (numrotation as i8) );
    
            if roty<rotx && roty>=-rotx {
                
                let sidelength = rotx * 2.0;
    
                //how much the position in the current section contributes to the total rotation
                let sectioncurrotation = ((roty / sidelength)+0.5 ) / 4.0;
    
                //add the amount this is already rotated to the amount to rotate
                let torotate = torotate + (numrotation as f32 * 0.25) + sectioncurrotation;
    
                //how many blocks (full 90 degree rotations) to apply to the newposprime2
                let blockstorotate = (torotate / 0.25).floor() as i8;


    
                //how much the section will contribute to the total rotation
                let sectiontorotate = torotate % 0.25;
    
    
                //bottom right corner
                let newposprime = (sidelength/2.0, -sidelength/2.0);
    
                //bottom right corner plus how much of a rotation is needed for this section * sidelength
                let newposprime2 = (newposprime.0, newposprime.1 + sectiontorotate * sidelength *4.0);
    
    
    
                let newpos = rotate_point_90_degrees_x_times(  newposprime2  , blockstorotate);
    
                return newpos;
    
            };
        };

        panic!("something went wrong");
    }
    


    fn rotate_point_90_degrees_clockwise( point: (f32,f32)) -> (f32,f32){
        //90 degrees clockwise rotation matrix (negative)
        //xprime = (0,  1)  = point.1
        //yprime = (-1, 0)  = -point.0
    
        return (point.1, -point.0);
    }
    
    fn rotate_point_90_degrees_counter_clockwise( point: (f32,f32)) -> (f32,f32){
        //90 degrees counter clockwise rotation matrix (positive)
        //xprime = (0, -1) = -point.1
        //yprime = (1,  0) = point.0
    
        return (-point.1, point.0);    
    }
    
    
    //how much to rotate this point in the positive rotation direction (counter clockwise)
    fn rotate_point_90_degrees_x_times( mut point: (f32,f32), mut times: i8 ) -> (f32,f32){
    
        while times != 0{
    
            //rotate in the positive direction
            if times > 0{
    
                point = rotate_point_90_degrees_counter_clockwise(point);
    
                times += -1;
            }
    
            //rotate in the negative direction
            if times < 0{
    
                point = rotate_point_90_degrees_clockwise(point);
    
                times += 1;
            }
        }
    
    
        return point;
    }
}