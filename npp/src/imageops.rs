use crate::image::CudaImage;
use npp_sys::{nppiResize_8u_C3R, NppiInterpolationMode_NPPI_INTER_LINEAR, NppiRect, NppiSize};
use rustacuda::error::*;

pub fn resize(src: &CudaImage<u8>, dst: &mut CudaImage<u8>) -> Result<(), CudaError> {
    let src_size: NppiSize = NppiSize {
        width: src.width() as i32,
        height: src.height() as i32,
    };
    let dst_size: NppiSize = NppiSize {
        width: dst.width() as i32,
        height: dst.height() as i32,
    };
    let src_rect: NppiRect = NppiRect {
        x: 0,
        y: 0,
        width: src.width() as i32,
        height: src.height() as i32,
    };
    let dst_rect: NppiRect = NppiRect {
        x: 0,
        y: 0,
        width: dst.width() as i32,
        height: dst.height() as i32,
    };
    let src_ptr = unsafe {
        src.image_buf
            .borrow_mut()
            .as_device_ptr()
            .offset(src.layout.img_index as isize)
            .as_raw()
    };
    let dst_ptr = unsafe {
        dst.image_buf
            .borrow_mut()
            .as_device_ptr()
            .offset(dst.layout.img_index as isize)
            .as_raw_mut()
    };
    let status = unsafe {
        nppiResize_8u_C3R(
            src_ptr,
            src.layout.height_stride as i32,
            src_size,
            src_rect,
            dst_ptr,
            dst.layout.height_stride as i32,
            dst_size,
            dst_rect,
            NppiInterpolationMode_NPPI_INTER_LINEAR as i32,
        )
    };
    if status == 0 {
        Ok(())
    } else {
        Err(CudaError::UnknownError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cuda::initialize_cuda_device;
    use crate::image::Persistable;
    use image::io::Reader as ImageReader;
    use image::{ColorType, RgbImage};
    use std::convert::TryFrom;

    #[test]
    fn test_resize1() {
        let _ctx = initialize_cuda_device();
        let img_src = ImageReader::open("test_resources/DSC_0003.JPG")
            .unwrap()
            .decode()
            .unwrap();
        let img_layout_src = img_src.as_rgb8().unwrap().sample_layout();

        let mut cuda_dst = match img_layout_src.channels {
            3 => CudaImage::new(640, 480, ColorType::Rgb8),
            _ => Err(CudaError::UnknownError),
        }
        .unwrap();

        let cuda_src = CudaImage::try_from(img_src.as_rgb8().unwrap()).unwrap();

        resize(&cuda_src, &mut cuda_dst).unwrap();

        cuda_dst.save("resize1").unwrap();

        let img_dst = RgbImage::try_from(&cuda_dst).unwrap();

        let img_layout_dst = img_dst.sample_layout();

        assert_eq!(img_layout_dst.channels, img_layout_src.channels);
        assert_eq!(img_layout_dst.channel_stride, img_layout_src.channel_stride);
        assert_eq!(img_layout_dst.width, 640);
        assert_eq!(img_layout_dst.height, 480);
    }

    #[test]
    fn test_resize2() {
        let _ctx = initialize_cuda_device();
        let img_src = ImageReader::open("test_resources/DSC_0003.JPG")
            .unwrap()
            .decode()
            .unwrap();
        let img_layout_src = img_src.as_rgb8().unwrap().sample_layout();

        let mut cuda_dst = match img_layout_src.channels {
            3 => CudaImage::new(640, 480, ColorType::Rgb8),
            _ => Err(CudaError::UnknownError),
        }
        .unwrap();

        let cuda_src = CudaImage::try_from(img_src.as_rgb8().unwrap()).unwrap();
        let sub_cuda_src = cuda_src.sub_image(1722, 954, 510, 555).unwrap();

        resize(&sub_cuda_src, &mut cuda_dst).unwrap();

        sub_cuda_src.save("resize2").unwrap();

        let img_dst = RgbImage::try_from(&cuda_dst).unwrap();
        let img_layout_dst = img_dst.sample_layout();

        assert_eq!(img_layout_dst.channels, img_layout_src.channels);
        assert_eq!(img_layout_dst.channel_stride, img_layout_src.channel_stride);
        assert_eq!(img_layout_dst.width, 640);
        assert_eq!(img_layout_dst.height, 480);
    }

    #[test]
    fn test_resize3() {
        let _ctx = initialize_cuda_device();
        let img_src = ImageReader::open("test_resources/DSC_0003.JPG")
            .unwrap()
            .decode()
            .unwrap();
        let img_layout_src = img_src.as_rgb8().unwrap().sample_layout();

        let cuda_src = CudaImage::try_from(img_src.as_rgb8().unwrap()).unwrap();
        let sub_cuda_src1 = cuda_src.sub_image(1722, 954, 510, 555).unwrap();
        let mut sub_cuda_src2 = cuda_src.sub_image(10, 10, 510, 555).unwrap();

        resize(&sub_cuda_src1, &mut sub_cuda_src2).unwrap();

        cuda_src.save("resize3").unwrap();

        let img_dst = RgbImage::try_from(&sub_cuda_src2).unwrap();
        let img_layout_dst = img_dst.sample_layout();

        assert_eq!(img_layout_dst.channels, img_layout_src.channels);
        assert_eq!(img_layout_dst.channel_stride, img_layout_src.channel_stride);
        assert_eq!(img_layout_dst.width, 510);
        assert_eq!(img_layout_dst.height, 555);
    }
}
