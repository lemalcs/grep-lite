// Store n lines of context around a match, for instance:
// If we have a text of 5 lines and the match is at line 2
// then store line 1, 3 and 4 (line 2 is also included in results)
fn main() {
    let ctx_lines = 2;
    let needle =  "oo";
    let haystack = "\
Every fave, every shop,
bedroom window, public-house, and
dark square in a picture,
feverishly turned-in search of what?
It is the same with books.
What do you seek
through millions of pages?";

    // Holds line numbers where matches occur
    let mut tags:Vec<usize> = vec![]; 

    // Holds each context lines in a vector
    let mut ctx:Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate(){
        if line.contains(needle){
            tags.push(i);
        }
        
        // No explicit type signature is needed as it can be inferred 
        // via the definition of `ctx` variable
        let v = Vec::with_capacity(2*ctx_lines + 1);
        // Vec<T> performs best when a size hint is provided

        ctx.push(v);
    }

    if tags.is_empty(){
        return;
    }

    for(i, line) in haystack.lines().enumerate(){
        for(j,tag) in tags.iter().enumerate(){

            // Subtraction that returns 0 on integer underflow rather than
            // crashing the program (CPUs don't like send usize below zero)
            let lower_bound = tag.saturating_sub(ctx_lines);

            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i<=upper_bound ){
                // Copy `line` into a new String
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter(){

        // `ref line` informs the compiler that we want to borrow this value 
        // rather than move it
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i+1;
            println!("{}: {}",line_num, line);
        }
    }

}
