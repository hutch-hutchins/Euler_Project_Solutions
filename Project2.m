%Euler Project 2
syms w
syms x
syms y
syms z

w = 0;
x = 0;
y = 1;
z = 0;

while x+y <= 4000000
    w = x+y;
    if mod(w,2) == 0
        z = z + w;
    end
    x = y;
    y = w;
end 
z
    