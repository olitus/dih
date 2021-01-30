# 

# Python 2
python=$(python --version 2>&1)
if [[ $python == *"Python"* ]]; then
    echo $python
else
    echo "Python 2 - Not intalled"
fi

# Python 3
python3=$(python --version 2>&1)
if [[ $python3 == *"Python"* ]]; then
    echo $python3i
else
    echo "Python 3 - Not installed"
fi

# Template
temp=$(temp --version 2>&1)
if [[]]; then
    echo
else
    echo
fi
