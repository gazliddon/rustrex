require 'pp'


################################################################################
def to_addr str
    vals = str.split(//).collect do |mode|
        mode == "X"
    end

    Hash[[:I, :E, :X, :D, :IMM, :R].zip(vals)]
end

class AddrModes 

    attr_accessor :modes

    def initialize addr_str

        vals = addr_str.split(//).collect do |mode|
            mode == "X"
        end

        @modes = Hash[[:I, :E, :X, :D, :IMM, :R].zip(vals)]

        is_inherent = @modes[:I]

        is_non_inherrent = [:E, :X, :D, :IMM, :R].reduce(false) do |acc,k|
            acc = acc || (@modes[k] != false)
        end

        if is_inherent && is_non_inherrent then
            raise "error instruction is is_inherent and is_non_inherrent"
        end
    end

end

################################################################################

FLAG_EFFECTS = {
    "-" => :unaffected,
    "*" => :affacted,
    "0" => :set,
    "?" => :undefined,
    "E" => :dunno
}

FLAG_BITS = {
    :I => 1 << 0, 
    :H => 1 << 1, 
    :N => 1 << 2, 
    :Z => 1 << 3, 
    :V => 1 << 4, 
    :C => 1 << 5
}

def get_flag_mask hsh, ftype
    hsh = hsh.select {|k,v| v == ftype}

    hsh.reduce(0) do |mask, ( k,_ )|
        mask + FLAG_BITS[k]
    end
end

class Ins

    attr_accessor :mnenomic
    attr_accessor :op_code
    attr_accessor :cycles
    attr_accessor :desc
    attr_accessor :notes
    attr_accessor :flags

    def initialize mnenomic, op_code,  flags_str, addr_str, cycles, desc, notes

        self.mnenomic = mnenomic
        self.op_code = op_code
        self.cycles = cycles
        self.desc = desc
        self.notes = notes

        # Decode the instructions string

        vals = flags_str.split(//).collect do |fl|
            FLAG_EFFECTS[fl] || :error
        end

        keys = [:I, :H, :N, :Z, :V, :C]

        effects = Hash[keys.zip(vals)]

        set_mask        = get_flag_mask(effects, :set)
        reset_mask      = get_flag_mask(effects, :reset)
        affected_mask   = get_flag_mask(effects, :affacted)
        unaffected_mask = get_flag_mask(effects, :unaffected)
        undefined_mask  = get_flag_mask(effects, :undefined)

        self.flags = {:effects         => effects,
                      :set_mask        => set_mask,
                      :reset_mask      => reset_mask ,
                      :affected_mask   => affected_mask,
                      :unaffected_mask => unaffected_mask,
                      :undefined_mask  => undefined_mask }
    end
end

################################################################################
#

################################################################################
#

def old_way 

    re = /^\|(.*)\|(\h{2})\|(.{6})\|(.{6})\|(\d)\|([^\|]+)\|(.*)\s*\|/

    File.open("./utils/ins.txt", "r") do |infile|

        instructions = []

        while (line = infile.gets)

            m = re.match(line)

            if m then
                mnenomic     = m[1].strip
                op_code      = m[2].to_i(16)
                flags_str    = m[3]
                addr_str     = m[4]
                cycles       = m[5].to_i
                desc         = m[6].strip
                notes        = m[7].strip

                ins = Ins.new(mnenomic, op_code, flags_str, addr_str, cycles, desc, notes)
                addr_modes = AddrModes.new(addr_str)

                ret = {:instruction =>  ins,
                       :addr_modes  => addr_modes }

                instructions << ret
            end
        end
        instructions
    end
end

lines = File.readlines("./utils/ins2.txt")

re = /^ \|\s+([A-F0-9]{2})[^|]+\|\s+([^\s]+)\s+\|\s+(\w+)\s+\|\s+(\d+)\s+\|\s+(\d+)\s+\|\s+(.{5})/

lines = lines.collect do |line|
    m = re.match(line)
    if m then
        op_code = m[1].to_i(16)
        mnenomic = m[2]
        addr_mode = m[3]
        cycles = m[4]
        bytes = m[5]
        flags_str = m[6]
        puts "#{op_code} : #{mnenomic} : #{addr_mode} : #{cycles} : #{bytes} : #{flags_str}"
    end
end


