#!/usr/bin/env python3
import re
import glob
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
API_CTRL = ROOT / 'src' / 'controller' / 'api_controller.rs'
COLLECTION = ROOT / 'collections' / 'root.http'

def load_controller():
    txt = API_CTRL.read_text()
    const_re = re.compile(r'const\s+([A-Z0-9_]+):\s*&str\s*=\s*route!\("([^"]+)"\);')
    return const_re.findall(txt)

def find_request_dto(base_segment):
    # search for dto files containing base_segment
    for path in glob.glob(str(ROOT / 'src' / 'api_module' / '**' / '*_dto' / '*dto.rs'), recursive=True):
        if base_segment in path or base_segment+'s' in path:
            # return path
            return Path(path)
    # fallback: search any dto that contains base_segment in filename
    for path in glob.glob(str(ROOT / 'src' / 'api_module' / '**' / '*_dto' / '*dto.rs'), recursive=True):
        name = Path(path).stem
        if base_segment.replace('_','') in name.replace('_',''):
            return Path(path)
    return None

def parse_request_fields(dto_path):
    txt = dto_path.read_text()
    # find "pub struct .*Request {" start
    m = re.search(r'pub struct\s+(\w*Request)\s*\{', txt)
    if not m:
        return None
    start = m.end()
    # find matching closing brace for the struct
    body = txt[start:]
    brace_count = 1
    fields_txt = ''
    for i,ch in enumerate(body):
        if ch == '{': brace_count +=1
        elif ch == '}':
            brace_count -=1
            if brace_count==0:
                fields_txt = body[:i]
                break
    fields = []
    for line in fields_txt.splitlines():
        line = line.strip()
        if line.startswith('pub '):
            # pub name: Type,
            parts = line.split(':',1)
            if len(parts)<2: continue
            name = parts[0].replace('pub ','').strip()
            t = parts[1].rstrip(',').strip()
            fields.append((name,t))
    return fields

def sample_value_for_type(t):
    if 'Option' in t:
        inner = re.search(r'Option<\s*([^>]+)\s*>', t)
        if inner:
            return sample_value_for_type(inner.group(1))
        return None
    if 'i64' in t or 'i32' in t or 'u64' in t or 'usize' in t:
        return 1
    if 'Decimal' in t or 'f64' in t or 'f32' in t:
        return 1.23
    if 'DateTimeWithTimeZone' in t or 'DateTime' in t:
        return '2026-04-02T00:00:00Z'
    if 'bool' in t:
        return True
    if 'Json' in t:
        return {}
    if 'String' in t or 'str' in t:
        return f"sample_{'field'}"
    # fallback
    return "sample"

def build_body(fields):
    obj = {}
    for name,t in fields:
        obj[name] = sample_value_for_type(t)
    return obj

def main():
    consts = load_controller()
    header = COLLECTION.read_text()
    out_lines = [header.strip(), '\n### Generated API collection\n']

    for const_name, path in consts:
        # determine method
        if const_name.endswith('_LIST'):
            method = 'GET'
        elif const_name.endswith('_BY_ID'):
            method = 'GET'
        elif const_name.endswith('_DELETE'):
            method = 'DELETE'
        elif const_name.endswith('_UPDATE'):
            method = 'PATCH'
        else:
            # default create
            method = 'PUT'

        # derive base segment
        seg = path.strip('/').split('/')[1] if path.strip('/').count('/')>=1 else path.strip('/')
        # try to find dto
        dto = find_request_dto(seg)
        body = None
        if dto:
            fields = parse_request_fields(dto)
            if fields:
                body = build_body(fields)

        # compose request
        if method in ('GET','DELETE') and '{:id}' in path:
            url = f"{{@base}}{path.replace('{:id}','/1')}"
        elif method in ('GET','DELETE') and '{:id}' not in path and path.endswith('s'):
            url = f"{{@base}}{path}"
        elif method in ('GET','DELETE') and '{:id}' not in path:
            # include id as query param
            url = f"{{@base}}{path}?id=1"
        else:
            url = f"{{@base}}{path}"

        out_lines.append(f"### {const_name} {method} {path}")
        out_lines.append(f"{method} {url}")
        if method in ('PUT','PATCH') and body is not None:
            out_lines.append("Content-Type: {@contentType}")
            out_lines.append("")
            out_lines.append(json.dumps(body, indent=2, ensure_ascii=False))
        out_lines.append('\n')

    COLLECTION.write_text('\n'.join(out_lines))

if __name__ == '__main__':
    main()
