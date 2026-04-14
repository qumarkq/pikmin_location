// 讓 TypeScript 認識 CSS 檔案
declare module "*.css" {
  const content: any;
  export default content;
}

// 如果未來有匯入圖片需求，也可以順便加上
declare module "*.png";
declare module "*.svg";
declare module "*.jpg";