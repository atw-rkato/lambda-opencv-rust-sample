use anyhow::Result;
use opencv::core::{Mat, Point, Scalar, Vector};
use opencv::imgcodecs::{imdecode, imencode};
use opencv::imgproc;
use opencv::imgproc::{
    contour_area, cvt_color, polylines, threshold, CHAIN_APPROX_NONE, COLOR_BGR2GRAY,
    RETR_EXTERNAL, THRESH_BINARY,
};
use opencv::types::VectorOfMat;

pub(crate) fn draw_contours(file: &[u8], ext: &str) -> Result<Vec<u8>> {
    let img = imdecode(&Mat::from_slice(file)?, 1)?;
    let gray = {
        let mut gray = Mat::default();
        cvt_color(&img, &mut gray, COLOR_BGR2GRAY, 0)?;
        gray
    };
    let bin_img = {
        let mut bin_img = Mat::default();
        threshold(&gray, &mut bin_img, 50., 255., THRESH_BINARY)?;
        bin_img
    };
    let contours = {
        let mut contours = VectorOfMat::new();
        imgproc::find_contours(
            &bin_img,
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_NONE,
            Point::default(),
        )?;
        contours
    };

    let mut img = img;
    for c in &contours {
        if contour_area(&c, false)? < 100. {
            continue;
        }
        polylines(&mut img, &c, true, Scalar::new(0., 255., 0., 1.), 5, 8, 0)?;
    }

    let mut buf = Vector::new();
    imencode(&format!(".{ext}"), &img, &mut buf, &Vector::new())?;
    Ok(buf.to_vec())
}

#[cfg(test)]
mod tests {
    use std::error;
    use std::ffi::OsStr;
    use std::path::Path;

    use opencv::core::{Mat, Scalar, Size, BORDER_DEFAULT};
    use opencv::imgcodecs::{imread, imwrite};
    use opencv::imgproc;
    use opencv::imgproc::{
        contour_area, cvt_color, gaussian_blur, polylines, threshold, CHAIN_APPROX_NONE,
        CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, COLOR_BGR2HSV, RETR_EXTERNAL, THRESH_BINARY,
    };
    use opencv::types::VectorOfMat;

    #[test]
    #[ignore]
    fn tulip() -> Result<(), Box<dyn error::Error>> {
        let filename = "img/tulip.jpg";
        let file = Path::new(filename);
        let dir = file.parent().and_then(Path::to_str).unwrap();
        let stem = file.file_stem().and_then(OsStr::to_str).unwrap();
        let ext = file.extension().and_then(OsStr::to_str).unwrap();

        let mut img = imread(filename, 1)?;
        let mut img_hsv = Mat::default();
        cvt_color(&img, &mut img_hsv, COLOR_BGR2HSV, 0)?;
        let mut img_hsv_blur = Mat::default();
        gaussian_blur(
            &img_hsv,
            &mut img_hsv_blur,
            Size {
                height: 9,
                width: 9,
            },
            3.,
            0.,
            BORDER_DEFAULT,
        )?;

        let mut hsv = VectorOfMat::new();
        opencv::core::split(&img_hsv_blur, &mut hsv)?;
        let mut img_flowers = Mat::default();
        threshold(&hsv.get(0)?, &mut img_flowers, 140., 255., THRESH_BINARY)?;
        imwrite("img/tulips_mask.jpg", &img_flowers, &Default::default())?;

        let mut contours = VectorOfMat::new();
        imgproc::find_contours(
            &img_flowers,
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_SIMPLE,
            Default::default(),
        )?;

        for c in contours {
            polylines(&mut img, &c, true, Scalar::new(0., 255., 0., 1.), 10, 8, 0)?;
        }
        imwrite(
            &format!("{dir}/{stem}-contours.{ext}"),
            &img,
            &Default::default(),
        )?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn jellyfish() -> Result<(), Box<dyn error::Error>> {
        let filename = "img/jellyfish.jpg";
        let file = Path::new(filename);
        let dir = file.parent().and_then(Path::to_str).unwrap();
        let stem = file.file_stem().and_then(OsStr::to_str).unwrap();
        let ext = file.extension().and_then(OsStr::to_str).unwrap();

        let mut img = imread(filename, 1)?;
        let mut gray = Mat::default();
        cvt_color(&img, &mut gray, COLOR_BGR2GRAY, 0)?;
        let mut bin_img = Mat::default();
        threshold(&gray, &mut bin_img, 50., 255., THRESH_BINARY)?;

        let mut contours = VectorOfMat::new();
        imgproc::find_contours(
            &bin_img,
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_NONE,
            Default::default(),
        )?;

        for c in &contours {
            if contour_area(&c, false)? < 100. {
                continue;
            }
            polylines(&mut img, &c, true, Scalar::new(0., 255., 0., 1.), 5, 8, 0)?;
        }
        imwrite(
            &format!("{dir}/{stem}-contours.{ext}"),
            &img,
            &Default::default(),
        )?;
        Ok(())
    }
}
