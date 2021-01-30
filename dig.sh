
# Python 2
python=$(python --version)
if [[ $python == *""* ]]; then
    echo $python
fi

# Python 3
python3=$(python3 --version)
if [[ $python == *""* ]]; then
    echo $python
fi
