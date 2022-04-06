COUNTRIES=%w[PL UK UA IT DE US]
ITEM_PER_COUNTRY = (ENV['ITEM_PER_COUNTRY'] || 3).to_i

total = 0

puts "PublicationCountry, BookId, AuthorAge, Pages, PublicationDate, AuthorNationality"
output = []

COUNTRIES.each { |c|
  # puts "country: #{c}"
  ITEM_PER_COUNTRY.times {|i|
    f1, f2, f3 = rand(0..10), rand(1..100), rand(500..1000)
    ac = COUNTRIES.sample
    output << "#{c}, #{total}, #{f1}, #{f2}, #{f3}, #{ac}"
    total += 1

  }
}

output.shuffle!

output.each {|l|
  puts l
}
