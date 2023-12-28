import "slick-carousel/slick/slick.css";
import "slick-carousel/slick/slick-theme.css";
import React, { Component, ReactNode } from "react";
import Slider from "react-slick";

interface CarouselProps {
    children?: ReactNode;
}

class Carousel extends Component<CarouselProps> {
    render() {
        const settings = {
            className: "center",
            infinite: true,
            centerPadding: "60px",
            slidesToShow: 3,
            swipeToSlide: true,
            variableWidth: true,
            // responsive: [
            //     {
            //         breakpoint: 1024,
            //         settings: {
            //             slidesToShow: 3,
            //             slidesToScroll: 3,
            //             infinite: true,
            //             dots: true
            //         }
            //     },
            //     {
            //         breakpoint: 600,
            //         settings: {
            //             slidesToShow: 2,
            //             slidesToScroll: 2,
            //             initialSlide: 2
            //         }
            //     },
            //     {
            //         breakpoint: 480,
            //         settings: {
            //             slidesToShow: 1,
            //             slidesToScroll: 1
            //         }
            //     }
            // ]
        };

        return (
            <div className="carousel-container" >
                <Slider {...settings}>
                    {React.Children.map(this.props.children, child => (
                        <div>{child}</div>
                    ))}
                </Slider>
            </div>
        );
    }
}

export { Carousel };
