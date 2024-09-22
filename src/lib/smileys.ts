// NOTE: this will be trash, and I won't refactor it xD
type Rating =
  | 'very_dissatisfied'
  | 'dissatisfied'
  | 'neutral'
  | 'satisfied'
  | 'very_satisfied';

const very_dissatisfied = `
    <svg
      data-name="Layer 5"
      id="b04fa8c6-b677-453e-b7d5-d4b16403edad"
      viewBox="0 0 344.69 344.69"
      xmlns="http://www.w3.org/2000/svg"
      ><defs
        ><style>
          .aa30f607-c9fa-4afd-a225-f87aab4800c9 {
            fill: #f8203d;
          }
          .f9fa5bad-ad74-4576-b3d4-4efd4b47a526 {
            fill: #12100e;
          }
          .a409166b-e296-4558-a761-23834b4414bc {
            fill: #d61b35;
          }
        </style></defs
      ><title>not-satisfied</title><circle
        class="aa30f607-c9fa-4afd-a225-f87aab4800c9"
        cx="180.26"
        cy="179.11"
        r="172.34"
        transform="translate(-81.77 173.15) rotate(-45)"
      ></circle><path
        class="f9fa5bad-ad74-4576-b3d4-4efd4b47a526"
        d="M247.59,114.11l-33,20.68a6.12,6.12,0,1,0,6.51,10.37h0V167.5a6.13,6.13,0,0,0,12.25,0V140a6.16,6.16,0,0,0-.43-2.24l21.13-13.26a6.12,6.12,0,0,0-6.51-10.37Z"
        transform="translate(-7.92 -6.77)"
      ></path><path
        class="f9fa5bad-ad74-4576-b3d4-4efd4b47a526"
        d="M126.29,140V167.5a6.13,6.13,0,0,0,12.25,0V145.16h0a6.12,6.12,0,0,0,6.51-10.37l-33-20.68a6.12,6.12,0,1,0-6.51,10.37l21.13,13.26A6.16,6.16,0,0,0,126.29,140Z"
        transform="translate(-7.92 -6.77)"
      ></path><path
        class="f9fa5bad-ad74-4576-b3d4-4efd4b47a526"
        d="M134.1,248.77a6.13,6.13,0,0,0,8.34,2.33A75.57,75.57,0,0,1,218,252a6.12,6.12,0,0,0,6.25-10.53,87.83,87.83,0,0,0-87.78-1A6.12,6.12,0,0,0,134.1,248.77Z"
        transform="translate(-7.92 -6.77)"
      ></path><path
        class="a409166b-e296-4558-a761-23834b4414bc"
        d="M321.05,79.82a171.34,171.34,0,0,1,16.37,73.26c0,95-77.31,172.35-172.34,172.35A172.26,172.26,0,0,1,24.29,252.38c27.59,58.49,87.12,99.08,156,99.08,95,0,172.34-77.32,172.34-172.35A171.37,171.37,0,0,0,321.05,79.82Z"
        transform="translate(-7.92 -6.77)"
      ></path></svg
    >`;
const dissatisfied = `
    <svg
      data-name="Layer 4"
      id="b0ac1aa4-1b63-4b75-95a5-cf9e3c4db033"
      viewBox="0 0 344.69 344.69"
      xmlns="http://www.w3.org/2000/svg"
      ><defs
        ><style>
          .b62fd0db-9100-41fe-87d2-b06e871364b5 {
            fill: #d84c4a;
          }
          .bdb27d82-0ef9-4201-9f25-838d634c6de8 {
            fill: #12100e;
          }
          .f5d876d7-0e2e-4496-be59-1a6db56b45f7 {
            fill: #e01e39;
          }
        </style></defs
      ><title>slightly-satisfied</title><circle
        class="b62fd0db-9100-41fe-87d2-b06e871364b5"
        cx="179.21"
        cy="178.51"
        r="172.34"
        transform="translate(-80.6 172.84) rotate(-45)"
      ></circle><path
        class="bdb27d82-0ef9-4201-9f25-838d634c6de8"
        d="M105.68,153.05a21.16,21.16,0,0,1,36.65,0,6.12,6.12,0,0,0,10.6-6.13,33.41,33.41,0,0,0-57.85,0,6.12,6.12,0,0,0,10.6,6.13Z"
        transform="translate(-6.87 -6.16)"
      ></path><path
        class="bdb27d82-0ef9-4201-9f25-838d634c6de8"
        d="M237,130.23a33.5,33.5,0,0,0-28.92,16.69,6.12,6.12,0,0,0,10.6,6.13,21.16,21.16,0,0,1,36.65,0,6.12,6.12,0,0,0,10.6-6.13A33.51,33.51,0,0,0,237,130.23Z"
        transform="translate(-6.87 -6.16)"
      ></path><path
        class="bdb27d82-0ef9-4201-9f25-838d634c6de8"
        d="M180.56,216.37a87.87,87.87,0,0,0-76,44,6.12,6.12,0,0,0,10.62,6.1,75.34,75.34,0,0,1,130.59-.2,6.12,6.12,0,1,0,10.6-6.13A87.88,87.88,0,0,0,180.56,216.37Z"
        transform="translate(-6.87 -6.16)"
      ></path><path
        class="f5d876d7-0e2e-4496-be59-1a6db56b45f7"
        d="M320,79.22a171.31,171.31,0,0,1,16.37,73.26c0,95-77.31,172.34-172.34,172.34a172.26,172.26,0,0,1-140.79-73c27.59,58.5,87.13,99.08,156,99.08,95,0,172.35-77.31,172.35-172.34A171.33,171.33,0,0,0,320,79.22Z"
        transform="translate(-6.87 -6.16)"
      ></path></svg
    >`;
const neutral = `
    <svg
      data-name="Layer 3"
      id="ab67ae85-2863-48ff-9b1d-7da618c74e02"
      viewBox="0 0 344.69 344.69"
      xmlns="http://www.w3.org/2000/svg"
      ><defs
        ><style>
          .ffcb7ffd-2c01-483d-bd0a-ed8f0843546d {
            fill: #fdc202;
          }
          .b7fa69d4-e1fd-431b-9cf4-7fa916639e90 {
            fill: #12100e;
          }
          .e4697f4f-d4dc-4429-9360-9ab186e05ecd {
            fill: #fff;
          }
          .bd079e85-9f8e-4a87-a2f2-1ccf740ddc88 {
            fill: #fda502;
          }
        </style></defs
      ><title>moderately-satisfied</title><circle
        class="ffcb7ffd-2c01-483d-bd0a-ed8f0843546d"
        cx="180.26"
        cy="179.51"
        r="172.34"
        transform="translate(-82.05 172.88) rotate(-45)"
      ></circle><path
        class="b7fa69d4-e1fd-431b-9cf4-7fa916639e90"
        d="M250.86,229.22h0l-70.58.21h-70.6a6.13,6.13,0,0,0,0,12.25h70.62l70.6-.21a6.13,6.13,0,0,0,0-12.25Z"
        transform="translate(-7.92 -7.16)"
      ></path><circle
        class="e4697f4f-d4dc-4429-9360-9ab186e05ecd"
        cx="235.7"
        cy="131.74"
        r="33.77"
      ></circle><path
        class="b7fa69d4-e1fd-431b-9cf4-7fa916639e90"
        d="M248.54,147.69a4.9,4.9,0,0,1,8.24-3.62,14.15,14.15,0,1,0-6.52,7.32A4.86,4.86,0,0,1,248.54,147.69Z"
        transform="translate(-7.92 -7.16)"
      ></path><circle
        class="e4697f4f-d4dc-4429-9360-9ab186e05ecd"
        cx="108.98"
        cy="131.74"
        r="33.77"
      ></circle><path
        class="b7fa69d4-e1fd-431b-9cf4-7fa916639e90"
        d="M121.81,147.69a4.9,4.9,0,0,1,8.24-3.62,14.15,14.15,0,1,0-6.52,7.32A4.89,4.89,0,0,1,121.81,147.69Z"
        transform="translate(-7.92 -7.16)"
      ></path><path
        class="bd079e85-9f8e-4a87-a2f2-1ccf740ddc88"
        d="M321.05,80.22a171.31,171.31,0,0,1,16.37,73.26c0,95-77.31,172.34-172.34,172.34a172.26,172.26,0,0,1-140.79-73c27.59,58.5,87.12,99.08,156,99.08,95,0,172.34-77.31,172.34-172.34A171.32,171.32,0,0,0,321.05,80.22Z"
        transform="translate(-7.92 -7.16)"
      ></path></svg
    >`;
const satisfied = `
  <svg
    data-name="Layer 2"
    id="bcaef541-fa11-4643-a093-0f70079efe15"
    viewBox="0 0 344.69 344.69"
    xmlns="http://www.w3.org/2000/svg"
    ><defs
      ><style>
        .e24c63fd-1194-4cc4-a4b8-0660ed8b5612 {
          fill: #7daf44;
        }
        .bf961c09-bbbc-46d4-9cc1-4febc3cc8c6f {
          fill: #12100e;
        }
        .b2e1e5d7-8dd3-420f-8f8f-6cdd9a931cc6 {
          fill: #449d49;
        }
      </style></defs
    ><title>very-satisfied</title><circle
      class="e24c63fd-1194-4cc4-a4b8-0660ed8b5612"
      cx="179.12"
      cy="178.51"
      r="172.34"
      transform="translate(-80.54 172.78) rotate(-45)"
    ></circle><path
      class="bf961c09-bbbc-46d4-9cc1-4febc3cc8c6f"
      d="M104.29,154a21.16,21.16,0,0,1,36.65,0,6.13,6.13,0,0,0,5.31,3,6.12,6.12,0,0,0,5.29-9.19,33.41,33.41,0,0,0-57.85,0,6.12,6.12,0,0,0,10.6,6.14Z"
      transform="translate(-6.78 -6.16)"
    ></path><path
      class="bf961c09-bbbc-46d4-9cc1-4febc3cc8c6f"
      d="M235.62,131.16a33.54,33.54,0,0,0-28.93,16.69,6.12,6.12,0,0,0,10.6,6.14,21.16,21.16,0,0,1,36.65,0,6.12,6.12,0,1,0,10.6-6.14A33.52,33.52,0,0,0,235.62,131.16Z"
      transform="translate(-6.78 -6.16)"
    ></path><path
      class="bf961c09-bbbc-46d4-9cc1-4febc3cc8c6f"
      d="M252.77,225.44a6.13,6.13,0,0,0-8.36,2.26,75.34,75.34,0,0,1-130.59.2,6.12,6.12,0,1,0-10.6,6.13A87.59,87.59,0,0,0,255,233.8,6.13,6.13,0,0,0,252.77,225.44Z"
      transform="translate(-6.78 -6.16)"
    ></path><path
      class="b2e1e5d7-8dd3-420f-8f8f-6cdd9a931cc6"
      d="M319.91,79.22a171.31,171.31,0,0,1,16.37,73.26c0,95-77.31,172.34-172.34,172.34a172.25,172.25,0,0,1-140.79-73,172.54,172.54,0,0,0,156,99.08c95,0,172.34-77.31,172.34-172.34A171.4,171.4,0,0,0,319.91,79.22Z"
      transform="translate(-6.78 -6.16)"
    ></path></svg
    >`;
const very_satisfied = `
    <svg
      data-name="Layer 1"
      id="b29c8562-0bb0-493e-96e4-e230e4a7227c"
      viewBox="0 0 344.69 344.69"
      xmlns="http://www.w3.org/2000/svg"
      ><defs
        ><style>
          .a8119c18-abd4-4142-a915-b817634c2df7 {
            fill: #449d4a;
          }
          .e65b3b5b-0de6-4e65-8032-98a45b57544c {
            fill: #12100e;
          }
          .e4d70b7b-85e6-4be8-8af5-1c4606a9e8e2 {
            fill: #fff;
          }
          .ec346200-a4b9-4027-a7eb-56bc28cfd73f {
            fill: #2a702f;
          }
        </style></defs
      ><title>very-much-satisfied</title><circle
        class="a8119c18-abd4-4142-a915-b817634c2df7"
        cx="179.12"
        cy="178.03"
        r="172.34"
        transform="translate(-80.2 173.11) rotate(-45)"
      ></circle><path
        class="e65b3b5b-0de6-4e65-8032-98a45b57544c"
        d="M104.29,152a21.16,21.16,0,0,1,36.65,0,6.13,6.13,0,1,0,10.61-6.13,33.42,33.42,0,0,0-57.86,0,6.12,6.12,0,1,0,10.6,6.13Z"
        transform="translate(-6.78 -5.69)"
      ></path><path
        class="e65b3b5b-0de6-4e65-8032-98a45b57544c"
        d="M235.62,129.19a33.51,33.51,0,0,0-28.93,16.69,6.12,6.12,0,1,0,10.6,6.13,21.16,21.16,0,0,1,36.65,0,6.12,6.12,0,1,0,10.6-6.13A33.5,33.5,0,0,0,235.62,129.19Z"
        transform="translate(-6.78 -5.69)"
      ></path><path
        class="e4d70b7b-85e6-4be8-8af5-1c4606a9e8e2"
        d="M249.72,205h0l-141.21.3a6.13,6.13,0,0,0-5.66,8.42c15.49,38.31,44.69,62.1,76.22,62.1S240,251.91,255.4,213.4a6.12,6.12,0,0,0-5.68-8.4Z"
        transform="translate(-6.78 -5.69)"
      ></path><path
        class="ec346200-a4b9-4027-a7eb-56bc28cfd73f"
        d="M319.91,78.74A171.35,171.35,0,0,1,336.28,152c0,95-77.31,172.34-172.34,172.34A172.25,172.25,0,0,1,23.15,251.3c27.58,58.49,87.12,99.08,156,99.08,95,0,172.34-77.32,172.34-172.35A171.44,171.44,0,0,0,319.91,78.74Z"
        transform="translate(-6.78 -5.69)"
      ></path></svg
    >`;
const ratings: Record<Rating, string> = {
  very_dissatisfied,
  dissatisfied,
  neutral,
  satisfied,
  very_satisfied,
};

const get_smiley = (rating: Rating): string => {
  return ratings[rating];
};

export { get_smiley };
export type { Rating };
