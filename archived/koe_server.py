from http.server import BaseHTTPRequestHandler, HTTPServer
import subprocess
import json
import logging

from koe import get_result_arr

class S(BaseHTTPRequestHandler):
    def _set_response(self):
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin","*")
        self.send_header("Access-Control-Allow-Methods","*")
        self.send_header("Access-Control-Allow-Headers","*")
        self.end_headers()

    def do_OPTIONS(self):
        self.send_response(200, "ok")
        self._set_response()

    # def do_GET(self):
    #     logging.info("GET request,\nPath: %s\nHeaders:\n%s\n", str(self.path), str(self.headers))
    #     self._set_response()
    #     self.wfile.write("GET request for {}".format(self.path).encode('utf-8'))

    def do_POST(self):
        content_length = int(self.headers['Content-Length']) # <--- Gets the size of data
        post_data = self.rfile.read(content_length) # <--- Gets the data itself
        body = json.loads(post_data.decode('utf-8'))
        self._set_response()

        if 'koe_open' in body:
            subprocess.run(["python3", __file__[:__file__.rindex("/")] + "/koe.py", "--open", body['koe_open']])
            self.wfile.write("OK".encode('utf-8'))
        elif 'koe_list' in body:
            result_arr = get_result_arr(body['koe_list'])
            self.wfile.write(json.dumps(result_arr).encode('utf-8'))
        elif 'direct_open' in body:
            subprocess.run(["open", body["direct_open"]])
            self.wfile.write("OK".encode('utf-8'))
        elif 'get_data' in body:
            result_arr = []

            try:
                with open(__file__[:__file__.rindex("/")] + "/data.json", 'r') as f:
                    data_json = json.loads(f.read())
                    for e in data_json:
                        if e['code'] in body['get_data']:
                            result_arr.append(e)
            except:
                pass

            if len(result_arr) == 0:
                result_arr = data_json
            self.wfile.write(json.dumps(result_arr).encode('utf-8'))

        # logging.info("POST request,\nPath: %s\nHeaders:\n%s\n\nBody:\n%s\n",
        #         str(self.path), str(self.headers), post_data.decode('utf-8'))


def run(server_class=HTTPServer, handler_class=S, port=8080):
    logging.basicConfig(level=logging.INFO)
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    logging.info('Starting httpd...\n')
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()
    logging.info('Stopping httpd...\n')

if __name__ == '__main__':
    from sys import argv

    if len(argv) == 2:
        run(port=int(argv[1]))
    else:
        run()
