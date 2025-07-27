

use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

use gltf::buffer::Source;
use gltf::Gltf;

pub fn open_gltf(path: &str) -> Result<Gltf, Box<dyn StdError>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    Ok(gltf)
}

pub fn load_mesh_data(gltf: &Gltf) -> Vec<(Vec<[f32; 3]>, Vec<u32>)> {
    let mut meshes = Vec::new();
    
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            // Получаем reader с правильной обработкой источника буфера
            let reader = primitive.reader(|buffer| {
                let buffer = gltf.buffers().nth(buffer.index())?;
                match buffer.source() {
                    Source::Uri(uri) => {
                        // Загрузка из внешнего файла (реализуйте эту часть)
                        todo!("Implement external buffer loading")
                    }
                    Source::Bin => {
                        // Для встроенных бинарных данных (GLB)
                        gltf.blob.as_deref()
                    }
                }
            });
            
            // Получаем позиции вершин
            let positions: Vec<[f32; 3]> = reader.read_positions()
                .expect("Mesh has no positions")
                .collect();
            
            // Получаем индексы
            let indices = reader.read_indices()
                .map(|iter| iter.into_u32().collect())
                .unwrap_or_else(|| (0..positions.len() as u32).collect());
            
            meshes.push((positions, indices));
        }
    }
    
    meshes
}