// #![allow(warnings)]

mod triangle;
pub mod rasterizer1;
pub mod rasterizer2;
pub mod rasterizer3;
mod utils;
mod texture;
mod shader;

extern crate opencv;

use std::env;
use clap::{App, Arg};  
use std::io;  
use nalgebra::Vector3;
use opencv::core::Vector;
use utils::*;
use crate::shader::FragmentShaderPayload;
use crate::texture::Texture;

mod task1;
mod task2;
mod task3;
use task1::t1;
use task2::t2;
use task3::t3;

fn main(){
    // 定义命令行参数  
    let matches = App::new("选择任务")    
        .arg(  
            Arg::with_name("任务序号")  
                .short('i')  
                .long("index")  
                .help("任务序号")  
                .required(true)  // 设置为必需参数  
                .takes_value(true)  
                .validator(|s| s.parse::<u32>().map_err(|e| e.to_string())),  // 验证参数是否为u32类型  
        )  
        .arg(  
            Arg::with_name("输出文件名")  
                .short('n')  
                .long("name")  
                .help("输出文件名")  
                .takes_value(true)  
        )
        .arg(  
            Arg::with_name("渲染方式")  
                .short('m')  
                .long("method")  
                .help("渲染方式")  
                .takes_value(true)  
        )  
        .get_matches();
    let count: u32 = matches.value_of("任务序号").unwrap_or("1").parse().unwrap();  // 如果参数缺失或无法解析，程序会panic 
    let filename = String::from(matches.value_of("输出文件名").unwrap_or("output.png"));
    let method = String::from(matches.value_of("渲染方式").unwrap_or("normal"));

    let _ = match count{
        1 => t1(),
        2 => t2(),
        3 => t3(filename,method),
        _ => Ok(()),
    };
}