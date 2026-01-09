#!/usr/bin/env python3
"""
Project Flattener - Flatten and restore project files using .gitignore rules.

Usage:
    python utils/project_flattener.py flatten -s . -o snapshot.json
    python utils/project_flattener.py flatten -s . -o snapshot.txt --base64  # Compressed, copyable
    python utils/project_flattener.py restore -i snapshot.json -t ./restored
    python utils/project_flattener.py info -i snapshot.json
"""

import os, sys, json, hashlib, argparse, fnmatch, gzip, base64
from datetime import datetime
from pathlib import Path


def parse_gitignore(path):
    """Parse .gitignore file and return patterns."""
    if not os.path.exists(path):
        return []
    with open(path, 'r', encoding='utf-8') as f:
        return [l.strip() for l in f if l.strip() and not l.startswith('#')]


def match_gitignore(path, patterns, is_dir=False):
    """Check if path matches any gitignore pattern."""
    path = path.replace('\\', '/')
    if is_dir and not path.endswith('/'):
        path += '/'

    for pattern in patterns:
        neg = pattern.startswith('!')
        if neg:
            pattern = pattern[1:]

        if pattern.endswith('/'):
            pattern = pattern[:-1]
            if not is_dir:
                continue

        if pattern.startswith('/'):
            match = fnmatch.fnmatch(path, pattern[1:]) or fnmatch.fnmatch(path, pattern[1:] + '/*')
        else:
            match = (fnmatch.fnmatch(path, pattern) or
                    fnmatch.fnmatch(path, '*/' + pattern) or
                    fnmatch.fnmatch(path, pattern + '/*') or
                    fnmatch.fnmatch(path, '*/' + pattern + '/*') or
                    ('/' + pattern + '/') in ('/' + path + '/'))

        if match:
            return not neg
    return False


def is_binary(filepath):
    """Check if file is binary."""
    try:
        with open(filepath, 'rb') as f:
            chunk = f.read(8192)
            if b'\x00' in chunk:
                return True
            text_chars = bytearray({7,8,9,10,12,13,27} | set(range(0x20, 0x100)) - {0x7f})
            non_text = sum(1 for b in chunk if b not in text_chars)
            return non_text / len(chunk) > 0.3 if chunk else False
    except:
        return True


def fmt_size(size):
    """Format file size."""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size < 1024:
            return f"{size:.1f}{unit}"
        size /= 1024
    return f"{size:.1f}TB"


def flatten(source, output, verbose=False, use_base64=False, exclude_tests=False, max_size=None):
    """Flatten project to single JSON file or base64-encoded compressed text."""
    src = Path(source).resolve()
    if not src.is_dir():
        print(f"Error: Directory not found: {source}")
        return False

    patterns = parse_gitignore(str(src / '.gitignore'))
    if '.git' not in patterns and '.git/' not in patterns:
        patterns.append('.git/')

    print(f"Scanning: {src}")
    if patterns:
        print(f"Using .gitignore rules ({len(patterns)} patterns)")

    files = []
    for root, dirs, names in os.walk(src):
        rel = os.path.relpath(root, src)
        rel = '' if rel == '.' else rel

        dirs[:] = [d for d in dirs if not match_gitignore(
            os.path.join(rel, d) if rel else d, patterns, is_dir=True)]

        for name in names:
            rel_path = os.path.join(rel, name) if rel else name
            if match_gitignore(rel_path, patterns):
                continue
            # Skip test files if requested
            if exclude_tests and ('/test/' in rel_path or '/Test' in rel_path or 'Test.java' in rel_path):
                if verbose:
                    print(f"  [skip test] {rel_path}")
                continue
            full = os.path.join(root, name)
            if is_binary(full):
                if verbose:
                    print(f"  [skip binary] {rel_path}")
                continue
            files.append((full, rel_path))

    files.sort(key=lambda x: x[1])
    print(f"Found {len(files)} text files")

    index, contents, total = [], {}, 0

    for i, (full, rel) in enumerate(files, 1):
        try:
            with open(full, 'r', encoding='utf-8') as f:
                content = f.read()
        except UnicodeDecodeError:
            try:
                with open(full, 'r', encoding='latin-1') as f:
                    content = f.read()
            except:
                continue
        except:
            continue

        fid = str(i)
        size = len(content.encode('utf-8'))
        total += size
        index.append({"id": fid, "path": rel, "size": size,
                     "hash": hashlib.sha256(content.encode()).hexdigest()[:16]})
        contents[fid] = content
        if verbose:
            print(f"  [{i}/{len(files)}] {rel}")

    data = {
        "metadata": {"version": "1.0", "created_at": datetime.now().isoformat(),
                    "source": str(src), "total_files": len(index), "total_size": total},
        "index": index, "files": contents
    }

    Path(output).parent.mkdir(parents=True, exist_ok=True)

    if use_base64:
        # Compress with gzip and encode as base64 for copyable text
        json_str = json.dumps(data, ensure_ascii=False, separators=(',', ':'))
        compressed = gzip.compress(json_str.encode('utf-8'), compresslevel=9)
        b64_str = base64.b64encode(compressed).decode('ascii')

        # Split into multiple files if max_size specified
        if max_size and max_size > 0:
            output_base = Path(output).stem
            output_ext = Path(output).suffix or '.txt'
            output_dir = Path(output).parent
            output_dir.mkdir(parents=True, exist_ok=True)

            # Calculate header size (approximately)
            header_size = 200  # Reserve space for header
            content_per_file = max_size - header_size

            # Split base64 string into chunks
            total_parts = (len(b64_str) + content_per_file - 1) // content_per_file
            output_files = []

            for part_num in range(1, total_parts + 1):
                start = (part_num - 1) * content_per_file
                end = min(part_num * content_per_file, len(b64_str))
                chunk = b64_str[start:end]

                out_file = output_dir / f"{output_base}_part{part_num}{output_ext}"
                with open(out_file, 'w', encoding='utf-8') as f:
                    f.write(f"# Project Snapshot Part {part_num}/{total_parts} (base64-gzip)\n")
                    f.write(f"# Files: {len(index)}, Original: {fmt_size(total)}\n")
                    f.write(f"# To restore: cat *_part*.txt | python utils/project_pattener.py restore -i - -t <target>\n")
                    f.write(f"# Or: Concatenate all parts, then restore\n")
                    f.write("#\n")
                    # Write base64 in chunks of 76 chars
                    for i in range(0, len(chunk), 76):
                        f.write(chunk[i:i+76] + '\n')

                file_size = out_file.stat().st_size
                output_files.append((part_num, file_size, str(out_file)))

            print(f"\nDone! Files: {len(index)}, Size: {fmt_size(total)}")
            print(f"Split into {total_parts} parts:")
            for part_num, file_size, path in output_files:
                print(f"  Part {part_num}: {fmt_size(file_size)} -> {path}")
            print(f"\nTo restore: Concatenate all parts (copy-paste in order), then use restore command")
        else:
            # Write single file with header for easy identification
            with open(output, 'w', encoding='utf-8') as f:
                f.write("# Project Snapshot (base64-gzip compressed)\n")
                f.write(f"# Files: {len(index)}, Original: {fmt_size(total)}\n")
                f.write(f"# Created: {datetime.now().isoformat()}\n")
                f.write("# To restore: python utils/project_pattener.py restore -i <file> -t <target>\n")
                f.write("#\n")
                # Write base64 in chunks of 76 chars for readability
                for i in range(0, len(b64_str), 76):
                    f.write(b64_str[i:i+76] + '\n')

            output_size = Path(output).stat().st_size
            compression_ratio = (1 - output_size / total) * 100 if total > 0 else 0
            print(f"\nDone! Files: {len(index)}, Size: {fmt_size(total)} -> {fmt_size(output_size)}")
            print(f"Compression ratio: {compression_ratio:.1f}%")
            print(f"Output: {output} (base64-gzip, copyable text)")
    else:
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        print(f"\nDone! Files: {len(index)}, Size: {fmt_size(total)} -> {fmt_size(Path(output).stat().st_size)}")
        print(f"Output: {output}")

    return True


def restore(input_file, target, force=False, skip=False):
    """Restore project from JSON file or base64-encoded file."""
    try:
        with open(input_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if it's base64-encoded
        if content.startswith('# Project Snapshot'):
            # Extract base64 content (skip header lines)
            lines = [l for l in content.split('\n') if l and not l.startswith('#')]
            b64_str = ''.join(lines)
            compressed = base64.b64decode(b64_str)
            json_str = gzip.decompress(compressed).decode('utf-8')
            data = json.loads(json_str)
        else:
            data = json.loads(content)
    except Exception as e:
        print(f"Error: {e}")
        return False

    meta, index, files = data.get('metadata', {}), data.get('index', []), data.get('files', {})
    print(f"Snapshot: {meta.get('total_files', len(index))} files, {fmt_size(meta.get('total_size', 0))}")

    tgt = Path(target).resolve()
    existing = [i['path'] for i in index if (tgt / i['path']).exists()]

    if existing and not force and not skip:
        print(f"\n{len(existing)} files exist:")
        for f in existing[:5]:
            print(f"  - {f}")
        if len(existing) > 5:
            print(f"  ... and {len(existing) - 5} more")

        choice = input("\n[o]verwrite [s]kip [c]ancel: ").strip().lower()
        if choice == 'c':
            return False
        force, skip = choice == 'o', choice == 's'

    print(f"\nRestoring to: {tgt}")
    restored = skipped = errors = 0

    for item in index:
        path = tgt / item['path']
        if item['path'] in existing and skip:
            skipped += 1
            continue
        content = files.get(item['id'])
        if content is None:
            errors += 1
            continue
        try:
            path.parent.mkdir(parents=True, exist_ok=True)
            with open(path, 'w', encoding='utf-8') as f:
                f.write(content)
            restored += 1
        except:
            errors += 1

    print(f"\nDone! Restored: {restored}, Skipped: {skipped}, Errors: {errors}")
    return errors == 0


def info(input_file):
    """Show snapshot info."""
    try:
        with open(input_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if it's base64-encoded
        if content.startswith('# Project Snapshot'):
            lines = [l for l in content.split('\n') if l and not l.startswith('#')]
            b64_str = ''.join(lines)
            compressed = base64.b64decode(b64_str)
            json_str = gzip.decompress(compressed).decode('utf-8')
            data = json.loads(json_str)
        else:
            data = json.loads(content)
    except Exception as e:
        print(f"Error: {e}")
        return False

    meta, index = data.get('metadata', {}), data.get('index', [])

    print(f"\n{'='*50}")
    print(f"Snapshot: {input_file}")
    print(f"{'='*50}")
    print(f"Version:  {meta.get('version', '-')}")
    print(f"Created:  {meta.get('created_at', '-')}")
    print(f"Source:   {meta.get('source', '-')}")
    print(f"Files:    {meta.get('total_files', len(index))}")
    print(f"Size:     {fmt_size(meta.get('total_size', 0))}")

    ext_count = {}
    for item in index:
        ext = Path(item['path']).suffix.lower() or '(none)'
        ext_count[ext] = ext_count.get(ext, 0) + 1

    print(f"\nFile types:")
    for ext, count in sorted(ext_count.items(), key=lambda x: -x[1])[:10]:
        print(f"  {ext:15s} {count:4d}")

    dirs = set()
    for item in index:
        parts = Path(item['path']).parts[:-1]
        for i in range(min(3, len(parts))):
            dirs.add('/'.join(parts[:i+1]))

    if dirs:
        print(f"\nDirectories (top 3 levels):")
        for d in sorted(dirs)[:15]:
            print(f"  {d}/")

    print(f"{'='*50}\n")
    return True


def main():
    p = argparse.ArgumentParser(description='Project Flattener - using .gitignore rules')
    sub = p.add_subparsers(dest='cmd')

    f = sub.add_parser('flatten', help='Flatten project')
    f.add_argument('-s', '--source', required=True, help='Source directory')
    f.add_argument('-o', '--output', required=True, help='Output file')
    f.add_argument('-v', '--verbose', action='store_true')
    f.add_argument('--base64', action='store_true', help='Output as base64-gzip compressed text (copyable)')
    f.add_argument('--no-tests', action='store_true', help='Exclude test files')
    f.add_argument('-m', '--max-size', type=str, default=None, help='Max size per file for base64 output (e.g., 400k)')

    r = sub.add_parser('restore', help='Restore project')
    r.add_argument('-i', '--input', required=True, help='Input file')
    r.add_argument('-t', '--target', required=True, help='Target directory')
    r.add_argument('--force', action='store_true', help='Overwrite existing')
    r.add_argument('--skip', action='store_true', help='Skip existing')

    i = sub.add_parser('info', help='Show snapshot info')
    i.add_argument('-i', '--input', required=True, help='Input file')

    args = p.parse_args()

    if args.cmd == 'flatten':
        max_size = None
        if args.max_size:
            size_str = args.max_size.lower()
            multipliers = {'k': 1024, 'm': 1024*1024, 'g': 1024*1024*1024}
            if size_str[-1] in multipliers:
                max_size = int(float(size_str[:-1]) * multipliers[size_str[-1]])
            else:
                max_size = int(size_str)
        sys.exit(0 if flatten(args.source, args.output, args.verbose, args.base64, args.no_tests, max_size) else 1)
    elif args.cmd == 'restore':
        sys.exit(0 if restore(args.input, args.target, args.force, args.skip) else 1)
    elif args.cmd == 'info':
        sys.exit(0 if info(args.input) else 1)
    else:
        p.print_help()
        sys.exit(1)


if __name__ == '__main__':
    main()
