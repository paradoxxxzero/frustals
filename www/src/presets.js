export default {
  preset: "Newton",
  remembered: {
    Mandelbrot: {
      "0": {
        variant: 0,
        precision: 25,
        smooth: true,
        order: 2,
        julia_real: -0.8,
        julia_imaginary: 0.156,
        lightness: 1.0
      },
      "1": {
        x: -0.75,
        y: 0,
        scale: 1.5
      }
    },
    Submandelbrot: {
      "0": {
        variant: 0,
        precision: 600,
        smooth: true,
        order: 2,
        julia_real: -0.8,
        julia_imaginary: 0.156,
        lightness: 1.0383573243014395
      },
      "1": {
        x: -0.5508,
        y: -0.6266,
        scale: 0.0028
      }
    },
    "Multibrot 3": {
      "0": {
        variant: 0,
        precision: 25,
        smooth: true,
        order: 3,
        julia_real: -0.8,
        julia_imaginary: 0.156,
        lightness: 1.0
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.5
      }
    },
    Newton: {
      "0": {
        variant: 1,
        precision: 20,
        smooth: true,
        order: 2,
        julia_real: -0.8,
        julia_imaginary: 0.156,
        lightness: 1.0
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.5
      }
    },
    Julia: {
      "0": {
        variant: 2,
        precision: 2000,
        smooth: true,
        order: 2,
        julia_real: -0.8,
        julia_imaginary: 0.156,
        lightness: 5
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.25
      }
    },
    "Julia 1-φ": {
      "0": {
        variant: 2,
        precision: 20,
        smooth: true,
        order: 2,
        julia_real: -0.61803398875,
        julia_imaginary: 0,
        lightness: 1.5
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.25
      }
    },
    "Julia φ−2 + (φ−1)i": {
      "0": {
        variant: 2,
        precision: 1000,
        smooth: true,
        order: 2,
        julia_real: -0.38196601125,
        julia_imaginary: 0.61803398875,
        lightness: 5.0
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.25
      }
    },
    "Julia (-.835 -.2321i)": {
      "0": {
        variant: 2,
        precision: 500,
        smooth: true,
        order: 2,
        julia_real: -0.835,
        julia_imaginary: -0.2321,
        lightness: 7
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.25
      }
    },
    "Julia (-.8i)": {
      "0": {
        variant: 2,
        precision: 200,
        smooth: true,
        order: 2,
        julia_real: 0,
        julia_imaginary: -0.8,
        lightness: 4
      },
      "1": {
        x: 0,
        y: 0,
        scale: 1.25
      }
    }
  },
  closed: false,
  folders: {}
};
