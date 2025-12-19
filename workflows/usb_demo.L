{
  "name": "USB 设备列表演示",
  "description": "扫描 USB 设备（动态端口）-> 日志显示设备数量和第一个设备信息",
  "blocks": {
    "a0000001-0000-0000-0000-000000000001": {
      "id": "a0000001-0000-0000-0000-000000000001",
      "script_id": "usb.device_scanner",
      "position": { "x": 50, "y": 100 },
      "size": { "x": 180, "y": 100 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "vid_filter": "",
        "pid_filter": ""
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null,
      "dynamic_outputs": []
    },
    "a0000002-0000-0000-0000-000000000002": {
      "id": "a0000002-0000-0000-0000-000000000002",
      "script_id": "debug.logger",
      "position": { "x": 300, "y": 100 },
      "size": { "x": 180, "y": 160 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "level": "INFO",
        "tag": "USB"
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    }
  },
  "connections": {
    "c0000001-0000-0000-0000-000000000001": {
      "id": "c0000001-0000-0000-0000-000000000001",
      "from_block": "a0000001-0000-0000-0000-000000000001",
      "from_port": "count",
      "to_block": "a0000002-0000-0000-0000-000000000002",
      "to_port": "in1"
    }
  },
  "groups": {},
  "viewport": { "offset": { "x": 0, "y": 0 }, "zoom": 1.0 },
  "readonly": false
}

