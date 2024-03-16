type ColorObject = {
    DEFAULT: string;
    dark: string;
};
interface Theme {
    colors: {
        primary: ColorObject;
        secondary: ColorObject;
        accent: ColorObject;
        success: ColorObject;
        warning: ColorObject;
        error: ColorObject;
        info: ColorObject;
        neutral: ColorObject;
        background: ColorObject;
        text: ColorObject;
    };
    spacing: {
        small: number;
        medium: number;
        large: number;
    };
}
export declare const theme: Theme;
export {};
