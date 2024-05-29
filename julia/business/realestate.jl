################################################################
################################################################
# 현금흐름 할인법
# Discounted Cash Flow Method
# 1. Net Present Value
# 2. Internal Rate of Returns
################################################################
# [ Net Present Value ]
# Today's value of the expected cash flows - Today's value of invested cash
#
# This mean `the time value of money`, `hurdle rate`
################################################################
i = requireReturn = discountRate = 0
t = numberOfTimePeriods = 0
paidYearFromNow = 1
initialInvestment = 0

# cash flow from one project
cashInFlow = 1000
cashOutFlow = 500
cashFlow = 현금흐름 = cashInFlow - cashOutFlow

NPV = cashFlow / (paidYearFromNow  + i)^t - initialInvestment 

# Multiple Cash Flow
n = numberOfYear = 10
R_t = netCashFlow = cashInFlow - cashOutFlow
NPV2 = sum(R_t/(paidYearFromNow+i)^t for _ in 0:n)

"""
이자율 = interest rate
연준 이자율 = discount rate
"""
discountRate = 금리 = 5
value = 100/금리

적정가치 = 현금흐름 * value

println(적정가치)
################################################################
# 거래사례 - 비슷한 건축물이 얼마에 거래가 되었나
################################################################

