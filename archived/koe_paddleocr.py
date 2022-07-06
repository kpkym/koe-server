import os, json, sys
from pathlib import Path
from paddleocr import PaddleOCR


def ocr(path):
    ocr_result = PaddleOCR(use_angle_cls=False, lang="ch").ocr(path)
    result = [x[1][0] for x in ocr_result if x[1][1] > 0.5 ]

    main_name = Path(path).stem


    with open(f'{os.path.dirname(path)}/{main_name}.json', 'w', encoding='utf8') as json_file:
        json.dump({"k": main_name, "v": result}, json_file, ensure_ascii=False)

if __name__ == '__main__':
    ocr(sys.argv[1])
    print("DONE")
