use std::rc::Rc;

struct Kid {
    ball: Rc<Ball>,
}

struct Ball;

fn main() {
    {
        let foo = Rc::new("foo");
    }
    {
        let bar = Rc::new("bar");
        let second_bar = Rc::clone(&bar);
    }

    {
        let baz = Rc::new("baz");
        {
            let second_baz = Rc::clone(&baz);
        }
    }

    let kid_one = spawn_kid_with_new_ball();
    let kid_two = Kid {
        ball: Rc::clone(&kid_one.ball),
    };
    let kid_three = Kid{
        ball: Rc::clone(&kid_one.ball),
    };
}

fn spawn_kid_with_new_ball() -> Kid {
    let ball = Rc::new(Ball);
    Kid {
        ball: Rc::clone(&ball),
    }
}