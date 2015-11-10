%%project Euler Problem 1

syms n
n = 0;
for i= 1:999
    if mod(i,3)== 0 | mod(i,5)==0
       n = n + i;
    end
end

n