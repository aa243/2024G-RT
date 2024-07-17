import os
from PIL import Image

def convert_ppm_to_png(folder_path):
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.ppm'):
                # 构建完整的文件路径
                ppm_path = os.path.join(root, file)
                # 构建目标PNG文件的路径
                png_path = os.path.splitext(ppm_path)[0] + '.png'
                
                # 检查PNG文件是否已存在
                if not os.path.exists(png_path):
                    # 使用Pillow打开PPM文件
                    with Image.open(ppm_path) as img:
                        # 将图像保存为PNG格式
                        img.save(png_path)
                        print(f"Converted {ppm_path} to {png_path}")
                else:
                    print(f"{png_path} already exists. Skipping conversion.")

# 调用函数，替换以下路径为你的目标文件夹路径
convert_ppm_to_png("./")