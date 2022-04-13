use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{Context,G2d};

use crate::draw::draw_block;

// RGB设置和不透明度
const  SNAKE_COLOR :Color = [0.80,0.80,0.00,1.0];

#[derive(Copy,Clone,PartialEq)]
pub enum Direction{
    Up,
    Down,
    Right,
    Left,
}

impl Direction{
    pub fn opposite (&self) -> Direction{
        match *self {
            Direction::Up =>Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Right,
        }
    }
}

#[derive(Copy,Clone)]
struct Block{
    x:i32,
    y:i32,
}

pub struct Snake{
    direction: Direction,
    body:LinkedList<Block>,
    tail:Option<Block>,
}

impl Snake{
    // 关联函数
    pub fn new(x:i32, y:i32)->Snake{
        let mut body:LinkedList<Block> = LinkedList::new();

        body.push_back(Block{
            x :x+2,
            y,
        });
        body.push_back(Block{
            x:x+1,
            y,
        });
        body.push_back(Block{
            x,
            y,
        });

        Snake{
            direction: Direction::Right,
            body,
            tail:None,
        }
    }

    // 方法
    pub fn draw(&self, con:&Context, g:&mut G2d){
        for block in &self.body{
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    
    // 当前头的位置
    pub fn head_position(&self)->(i32,i32){
        // unwrap 简化match Option代码
        let head_position = self.body.front().unwrap();
        (head_position.x,head_position.y)
    }


    pub fn move_forward(&mut self,dir:Option<Direction>){
        match dir{
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) :(i32,i32)= self.head_position(); 

        let new_block = match self.direction{
            Direction::Up => Block{
                x:last_x,
                y:last_y-1,
            },
            Direction::Down => Block{
                x:last_x,
                y:last_y+1,
            },
            Direction::Left => Block{
                x:last_x-1,
                y:last_y,
            },
            Direction::Right => Block{
                x:last_x+1,
                y:last_y,
            },
        };
        // 变化方向
        self.body.push_front(new_block);
        // 将原有list后的块删除,removed_block是被删除的元素
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction{
        self.direction
    }

    // 头前进后的位置
    pub fn next_head(&self ,dir:Option<Direction>) -> (i32,i32){
        let (head_x,head_y) :(i32,i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        }

        match moving_dir {
            Direction::Up => (head_x,head_y-1),
            Direction::Down => (head_x,head_y+1),
            Direction::Left => (head_x-1,head_y),
            Direction::Right => (head_x+1,head_y),
        }
    }

    // 吃掉苹果后，尾巴会被推入链表尾部
    pub fn restore_tail(&mut self){
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk)
    }

    // 如果下一个方向撞到了自己，就返回false
    pub fn overlap_tail(&self, x:i32, y:i32) -> bool{
        let mut ch = 0;
        for block in &self.body{
            if x == block.x && y == block.y{
                return true;
            }

            ch += 1;
            
            if ch == self.body.len() -1 {
                break;
            }
        }
        return false;
    }

}