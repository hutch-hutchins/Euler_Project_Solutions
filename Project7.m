%Euler Problem 7

j = 0;

for i = 1:10^6
    if isprime(i)
        j = j+1;
        if j == 10001
            i
        end
    end
end