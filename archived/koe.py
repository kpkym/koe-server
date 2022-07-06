import subprocess
import sys, os
import operator
import json

def list_absolute_path(path):
    r = [f for f in os.listdir(path)]
    return [f'{path}/{x}' for x in r]

def get_result_arr(rj):
    result_arr = []
    path_arr = []

    path_arr.append(f"{os.path.expanduser('~')}/Downloads")
    path_arr.append(f"{os.path.expanduser('~')}/ooo/koe")
    
    for idx, val in enumerate(path_arr):
        # print(f"正在搜索[{idx + 1}/{len(path_arr)}]: {val}")
        out = subprocess.Popen(['fd', rj, val], stdout=subprocess.PIPE,  stderr=subprocess.STDOUT)
        stdout, stderr = out.communicate()

        result = stdout.decode("utf-8")
        if result:
            # 到这里表示有搜索结果
            result_arr.extend([{"type": "D" if os.path.isdir(x) else "F" , "path": x } for x in list(filter(None, result.split("\n")))])

    naskoe_dir_file = list_absolute_path(__file__[:__file__.rindex("/")] + "/nas_koe_data")
    naskoe_dir_file.sort(reverse=True)

    # print(f"读取文件: {naskoe_dir_file[0]}")
    with open(naskoe_dir_file[0], 'r') as f:
        nas_koe_json_arr = json.loads(f.read())
        # 不要文件
        nas_koe_json_arr = [a for a in nas_koe_json_arr if a['type'] == 'D' and rj in a['path']]
        for item in nas_koe_json_arr:
            item["path"] = "/Volumes/koe" + item["path"]
        result_arr.extend(nas_koe_json_arr)

    result_arr.sort(key=lambda x: len(x["path"]))
    return result_arr

def func(rj, is_open):
    result_arr = get_result_arr(rj)

    if not result_arr:
        print("不存在结果, 退出")
        return

    print("结果################")


    # 列出选项
    for idx, val in enumerate(result_arr):        
        type, path = operator.itemgetter('type', 'path')(val)
        print(f'{idx}[{type}]: {path}')

    # 如果是直接打开就打开第一个
    if is_open:
        subprocess.run(["open", result_arr[0]['path']])
        return

    try:
        idx = input()
        subprocess.run(["open", result_arr[int(idx)]['path']])
    except:
        return


if __name__ == '__main__':
    func(sys.argv[2], sys.argv[1] == '--open')
