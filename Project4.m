%Euler Project 4

i = 0;
j = 0;
x = 0;
s = 0;
a = [];

for i = 100:999
    for j = 100:999
        x = i * j;
        s = num2str(x);
        if s == fliplr(s)
            a = [a , x]; 
        end
    end
    end

max(a)
        