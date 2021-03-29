
use std::f32::consts::PI;



fn main() {
    
    
    
    let mut square : Vec<Vec<u32>> = Vec::new();
    let mut newsquare: Vec<Vec<u32>> = Vec::new();
    
    let mut total = 100;
    
    
    for x in 0..9{
        let mut newsquarevec = Vec::new();
        for y in 0..9{
            newsquarevec.push(888);
        }
        
        newsquare.push(newsquarevec);
    }
    
    
    for x in 0..9{
        
        let mut squarevec = Vec::new();
        
        for y in 0..9{
            
            squarevec.push(total);
            
            let posx = x - 4;
            let posy = y - 4;
            
            println!("theposses numb{:?} ,value pos{:?}",total, (posx,posy) );

            let newpos = ortho_rotate_point( (posx as f32, posy as f32) , 0.055);

            println!("angle and ortho {:?}", newpos);

            println!(" ");

            
            //println!("newposxandy {:?}", newpos );
            
            let newposx = newpos.0.round() + 4.0;
            let newposy = newpos.1.round() + 4.0;
            
            let vecx = newposx as usize;
            let vecy = newposy as usize;
            
            //println!("newposxandy {:?}", (vecx, vecy) );
            
            newsquare[vecx][vecy] = total;
            
            
            
            
            
            total += 1;
        };
        
        square.push(squarevec);
        
    };
    
    
    print_2d_vector(square);
    println!("");
    print_2d_vector(newsquare);
    
    
    


    use poloto::*;


    let mut data = Vec::new();

    for x in -500..500{

        let float = 0.001 * x as f32;

        data.push( [ float, float ] );

    }


    let mut s = poloto::plot("simple", "x", "y");

    s.scatter("", data.into_iter());

    

    //s.render(std::io::stdout()).unwrap();

    let file = std::fs::File::create("write_to_file.svg").unwrap();

    s.render(file);
    
    
    
}

fn print_2d_vector(vec: Vec<Vec<u32>>){
    
    for x in vec{
        for y in x{
            
            print!("  {:?}  ", y);
        }
        
        println!("");
    }
}




/*
fn positive_to_negative(radians: f32) -> f32{
    
    let mut radians = radians % (PI*2.0);

    if radians < 0.0{
        radians = - radians;
    }
    
    
    /*
    (( 2.0*( radians.cos().floor() ) ) + 1.0)
    */
    
    if radians <= (PI/2.0){
        return 1.0;
    }
    else if radians <= (1.5*PI) {
        return -1.0;
    }
    else{
        return 1.0;
    }
    
}

fn q_function( radians: f32 ) -> f32{
    
    return positive_to_negative(radians) * radians.tan();
}

fn r_function( radians: f32) -> f32{
    
    let result = q_function(radians);
    
    if result > 1.0{
        return 1.0;
    }
    if result < -1.0{
        return -1.0;
    }
    
    return result;
}

fn angle_ortho_distance_to_position(radians: f32, orthodist: f32) -> (f32,f32){
    
    //let radians = radians % (PI*2.0);
    
    let x = orthodist * r_function( radians + PI/2.0 );
    let y = orthodist * r_function( radians );
    
    return (x,y);
}
*/

/*

fn r_function(radians: f32) -> f32{

    let result = 2.0f32.sqrt() * radians.sin() ;

    if result > 1.0{
        return 1.0;
    }
    if result < -1.0{
        return -1.0;
    }
    
    return result;
}

fn angle_ortho_distance_to_position(radians: f32, orthodist: f32) -> (f32,f32){


    let x = orthodist* r_function( (radians +PI) / 2.0);
    let y = orthodist* r_function( radians/2.0 );


    return(x,y);
}




//I have a position
//I want its new position when its rotated
//
//divide


fn position_to_angle_and_ortho_distance(pos: (f32,f32)) -> (f32,f32){
    
    let orthonumber;
    
    //find the 
    if pos.0.abs() > pos.1.abs(){
        orthonumber = pos.0.abs();
    }
    else{
        orthonumber = pos.1.abs();
    }
    
    
    let mut angle = pos.1.atan2(pos.0);

    angle = angle * 2.0; // - PI/2.0;
    
    
    return (angle, orthonumber);
}

*/




//get the position its at
//rotate it by a certain amount

//some angle (portion of a full rotation)
//multiply it by 4* ortho distance
//
//get the ortho distance
//get the "angle" which is % progressed around in a circle



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



//rotate this point by this amount and return it
fn ortho_rotate_point( point: (f32,f32), torotate: f32) -> (f32,f32){



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

            println!("the sec rotation {:?}", sectioncurrotation);

            //add the amount this is already rotated to the amount to rotate
            let torotate = torotate + (numrotation as f32 * 0.25) + sectioncurrotation;

            println!("amount to rotate {:?}", torotate);

            //how many blocks (full 90 degree rotations) to apply to the newposprime2
            let blockstorotate = (torotate / 0.25).floor() as i8;

            println!("blocks to rotate {:?}", blockstorotate);



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

/*
fn position_to_angle_and_ortho_distance(pos: (f32,f32)) -> (f32,f32){
    
    let x = pos.0;
    let y = pos.1;

    
    let orthonumber = x.abs().max( y.abs() ).max(0.0);


    //how much of the circle is progressed
    let mut curangle;


    //make sure float comparison works like I think it does
    if x.abs() >= y.abs() && x > 0.0 {

        curangle = 12.5;
        curangle += y / orthonumber*8.0;
    }
    else if y.abs() > x.abs() && y > 0.0{

        curangle = 37.5;
        curangle += -x / orthonumber*8.0;
    }
    else if x.abs() > y.abs() && x < 0.0{

        curangle = 62.5;
        curangle += -y / orthonumber*8.0;
    }
    else{

        curangle = 87.5;
        curangle += x / orthonumber*8.0;
    }

    
    
    return (curangle, orthonumber);
}




fn angle_ortho_distance_to_position(angle: f32, orthodist: f32) -> (f32,f32){

    let angle = angle % 1.0;


    //angle is percentage of the circle completed
    //start counting at bottom right

    /*
    _ 0.25 (right)

    _ 0.5 (top)

    _ 0.75 (left)

    _ 1.00 (bottom)
    */


    let partangle = (angle % 25.0)/25.0;
    let partangledist = partangle * orthodist *2.0;

    let x;
    let y;

    if angle < 0.25{
        x = orthodist;
        y = partangledist - orthodist;
    }
    else if angle < 0.5{
        x = -partangledist + orthodist;
        y = orthodist;
    }
    else if angle < 0.75{
        x = -orthodist;
        y = -partangledist + orthodist;
    }
    else{
        x = -orthodist + partangledist;
        y = -orthodist;
    }



    return (x,y);


}

*/