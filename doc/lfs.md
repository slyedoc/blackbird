# Git LFS

## commands

To setup lfs in a repository:

```bash
git lfs install
git lfs track "*.ktx2"
```

## migrate

If you have already committed files, you can migrate them to lfs:

```bash
git lfs migrate import --include="*.ktx2"
```

## restore

```bash
git lfs untrack '*.png'
git rm --cached '*.png'
git add '*.png'
git commit -m "restore '*.png'' to git from lfs"
```